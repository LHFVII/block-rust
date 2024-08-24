use std::collections::HashMap;

use crate::domain::{transaction, Block};
use jammdb::{DB, Error};

use super::{Transaction, TxOutput};
const BLOCKS_BUCKET: &str = "blocks";

pub struct Blockchain {
    pub tip: Vec<u8>,
    pub db: DB,
    pub current_hash: Vec<u8>,
}

impl Blockchain {
    pub fn new(address: String, genesis_cb_data: String) -> Result<Self, Error> {
        let db = DB::open("blockchain.db")?;
        let tip = {
            let tx = db.tx(true)?;
    
            let result = match tx.get_bucket(BLOCKS_BUCKET) {
                Ok(bucket) => {
                    bucket.get(b"l")
                        .map(|data| data.kv().value().to_vec())
                        .unwrap_or_else(Vec::new)
                },
                Err(_) => {
                    let block_bucket = tx.create_bucket(BLOCKS_BUCKET)?;
                    let coinbase_tx = Transaction::new_coinbase_tx(address,genesis_cb_data);
                    let genesis = Block::new(vec![coinbase_tx], Vec::new());
                    let genesis_hash = genesis.hash.clone();
                    
                    let block_bytes = rmp_serde::to_vec(&genesis)
                        .map_err(|_| Error::IncompatibleValue)?;
                    
                    block_bucket.put(genesis.hash, block_bytes)?;
                    block_bucket.put(b"l".to_vec(), genesis_hash.clone())?;
                    
                    genesis_hash
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

    pub fn mine_block(&mut self, transactions: Vec<Transaction>) -> Result<bool, Error> {
        let tx = self.db.tx(true)?;
        let bucket = tx.get_bucket(BLOCKS_BUCKET)?;
        if let Some(data) = bucket.get(b"l") {
            let block: Block = rmp_serde::from_slice(data.kv().value()).ok()?;
            let new_block = Block::new(transactions, block.hash);
            let block_bytes = rmp_serde::to_vec(&new_block)
                        .map_err(|_| Error::IncompatibleValue)?;
            bucket.put(new_block.hash, block_bytes)?;
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

    pub fn find_unspent_transactions(address: String) -> Vec<Transaction>{
        vec![Transaction::new()]
    }

    pub fn find_utxo(address: String) -> Vec<TxOutput>{
        vec![Transaction::new()]
    }

    pub fn find_spendable_outputs() -> HashMap<String, Vec<u32>> {
        HashMap::<String, Vec<u32>>::new()
    }

    
}



