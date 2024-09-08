use std::{collections::HashMap, path::Path};
use crate::domain::{Transaction, Block, TxOutput};
use jammdb::{DB};
use std::error::Error;
use secp256k1::{SecretKey};


const BLOCKS_BUCKET: &str = "blocks";
const GENESIS_COINBASE_DATA: &str = "ALPHA";
const DB_PATH: &str = "blockchain.db";

pub struct Blockchain {
    pub tip: Vec<u8>,
    pub db: DB,
    pub current_hash: Vec<u8>,
}

impl Blockchain {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        if !Path::new(DB_PATH).exists() {
            return Err("Blockchain does not exist.".into())
        }
        let db = DB::open(DB_PATH)?;
        let tip = {
            let tx = db.tx(true)?;
            let result = match tx.get_bucket(BLOCKS_BUCKET) {
                Ok(bucket) => {
                    bucket.get(b"l")
                        .map(|data| data.kv().value().to_vec())
                        .unwrap_or_else(Vec::new)
                },
                Err(_) => {
                    println!("Error: Tip not found");
                    return Err("Tip not found".into())
                }
            };
            tx.commit()?;
            result
        };
        
        Ok(Blockchain { 
            tip: tip.clone(),
            current_hash: tip,
            db 
        })
    }

    pub fn create_blockchain(address: String) -> Result<Self, Box<dyn Error>>{
        if Path::new(DB_PATH).exists() {
            return Err("Blockchain already exists.".into())
        }
        let db = DB::open(DB_PATH)?;
        let tx = db.tx(true)?;
        let block_bucket = tx.create_bucket(BLOCKS_BUCKET)?;
        let coinbase_tx = Transaction::new_coinbase_tx(address,String::from(GENESIS_COINBASE_DATA));
        let genesis = Block::new(vec![coinbase_tx], Vec::new());
        let genesis_hash = genesis.hash.clone();
        let block_bytes = rmp_serde::to_vec(&genesis)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        block_bucket.put(genesis.hash, block_bytes)?;
        block_bucket.put(b"l".to_vec(), genesis_hash.clone())?;
        
        Ok(Blockchain{
            tip: genesis_hash.clone(),
            current_hash: genesis_hash,
            db: db.clone(),
        })
        
    }

    pub fn mine_block(&mut self, transactions: Vec<Transaction>) -> Result<bool, Box<dyn Error>> {
        let tx = self.db.tx(true)?;
        let bucket = tx.get_bucket(BLOCKS_BUCKET)?;
        if let Some(data) = bucket.get(b"l") {
            let block: Block = rmp_serde::from_slice(data.kv().value())
                .map_err(|e| Box::new(e) as Box<dyn Error>)?;
            let new_block = Block::new(transactions, block.hash);
            let block_bytes = rmp_serde::to_vec(&new_block)
                        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
            bucket.put(new_block.hash.clone(), block_bytes)?;
            bucket.put(b"l".to_vec(), new_block.hash)?;
        }
        tx.commit()?;
        Ok(true)
    }

    pub fn next(&mut self) -> Option<Block> {
        if self.current_hash.is_empty() {
            return None;
        }
        let tx = self.db.tx(false).ok()?;
        let bucket = tx.get_bucket(BLOCKS_BUCKET).ok()?;
        if let Some(data) = bucket.get(&self.current_hash) {
            let block: Block = rmp_serde::from_slice(data.kv().value()).ok()?;
            self.current_hash = block.prev_block_hash.clone();
            Some(block)
        } else {
            println!("Nothing was found");
            None
        }
    }

    pub fn find_unspent_transactions(&mut self, address: Vec<u8>) -> Vec<Transaction> {
        let mut unspent_txs: Vec<Transaction> = Vec::new();
        let mut spent_txos: HashMap<String, Vec<u8>> = HashMap::new();
        let mut current_block = self.next();
        while let Some(ref block) = current_block {
            for tx in &block.transactions {
                let current_tx = tx.clone();
                let tx_id = hex::encode(&current_tx.id);
                'outputs: for (out_idx, out) in current_tx.vout.iter().enumerate() {
                    if let Some(spent_outputs) = spent_txos.get(&tx_id) {
                        if spent_outputs.contains(&(out_idx as u8)) {
                            continue 'outputs;
                        }
                    }
            
                    if out.is_locked_with_key(address.clone()) {
                        unspent_txs.push(tx.clone());
                    }
                }
                if !tx.is_coinbase() {
                    for input in &tx.vin {
                        if input.uses_key(address.clone()) {
                            let in_tx_id = hex::encode(&input.txid);
                            spent_txos.entry(in_tx_id)
                                .or_insert_with(Vec::new)
                                .push(input.vout);
                        }
                    }
                }
            }
            if block.prev_block_hash.is_empty() {
                break;
            }
            current_block = self.next();
        }
    
        unspent_txs
    }

    pub fn find_utxo(&mut self,address: Vec<u8>) -> Vec<TxOutput>{
        let mut utxos = Vec::new();
        let unspent_txs = self.find_unspent_transactions(address.clone());
        for tx in unspent_txs{
            for out in tx.vout{
                if out.is_locked_with_key(address.clone()){
                    utxos.push(out);
                }
            }
        }
        utxos
    }

    pub fn find_spendable_outputs(&mut self, address: Vec<u8>, amount: u32) -> (u32,HashMap<String, Vec<u8>>) {
        let mut unspent_outputs:HashMap::<String, Vec<u8>> = HashMap::<String, Vec<u8>>::new();
        let unspent_txs = self.find_unspent_transactions(address.clone());
        let mut accumulated = 0;
        'work:
            for tx in unspent_txs{
                let id = hex::encode(tx.id);
                for (out_id,out) in tx.vout.iter().enumerate(){
                    if out.is_locked_with_key(address.clone()) && accumulated < amount {
                        accumulated += out.value;
                        unspent_outputs.entry(id.clone()).or_default().push(out_id as u8);

                        if accumulated >= amount{
                            break 'work;
                        }
                    }
                }
            }

        (1,HashMap::<String, Vec<u8>>::new())
    }

    pub fn find_transaction(&mut self,id: Vec<u8>)-> Result<Transaction, Box<dyn Error>>{
        let mut current_block = self.next();
    
        while let Some(block) = current_block {
            for tx in block.transactions{
                if tx.id == id{
                    return Ok(tx)
                }
            }
            if block.prev_block_hash.is_empty() {
                break;
            }
            current_block = self.next();
        }
        return Err("Transaction not found".into())
    }

    pub fn sign_transaction(&mut self, mut transaction: Transaction, private_key: &SecretKey) {
        let mut prev_txs = HashMap::new();
    
        for vin in &transaction.vin {
            match self.find_transaction(vin.txid.clone()) {
                Ok(prev_tx) => {
                    let tx_id_hex = hex::encode(&prev_tx.id);
                    prev_txs.insert(tx_id_hex, prev_tx);
                },
                Err(err) => {
                    panic!("Error finding previous transaction: {}", err);
                }
            }
        }
        transaction.sign(private_key, prev_txs);
    }  
}



