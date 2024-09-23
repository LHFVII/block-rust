use std::collections::HashMap;
use std::error::Error;
use hex::decode;
use serde::{Deserialize, Serialize};
use crate::domain::{Block, Blockchain};

#[derive(Clone)]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct TxOutput{
    pub value: u32,
    pub pubkey_hash: Vec<u8>
}

impl TxOutput{
    pub fn new(value: u32, address: String)-> Self{
        let mut txo = TxOutput{value:value, pubkey_hash:Vec::new()};
        txo.lock(address.into_bytes());
        return txo;
    }

    pub fn is_locked_with_key(&self, pubkey_hash: Vec<u8>) -> bool{
        return self.pubkey_hash == pubkey_hash;
    }

    pub fn lock(&mut self,address: Vec<u8>){
        let decoded = bs58::decode(address).into_vec().expect("Failed to decode address");
        self.pubkey_hash = decoded[1..21].to_vec();
    }
}
#[derive(Clone)]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct TxOutputs{
    pub outputs: Vec<TxOutput>
}

// Acts as a cache that is built from all blockchain transactions
pub struct UTXOSet{
    pub blockchain: Blockchain
}

const UTXO_BUCKET: &str = "UTXOSet";

impl UTXOSet{
    pub fn reindex(&mut self) -> Result<(), Box<dyn Error>>{
        let db = self.blockchain.db.clone();
        let tx = db.tx(true)?;
        tx.delete_bucket(UTXO_BUCKET);
        let block_bucket = tx.create_bucket(UTXO_BUCKET)?;
        let utxo = self.blockchain.find_utxo();
        for (tx_id, outs) in utxo{
            let key = decode(tx_id).unwrap();
            let outs_bytes = rmp_serde::to_vec(&outs)
                        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
            block_bucket.put(key, outs_bytes);
        }
        tx.commit()?;
        Ok(())
    }

    pub fn find_spendable_outputs(&self,pubkey_hash: Vec<u8>, amount: u64) -> (u64, HashMap<&str,Vec<u64>>){
        let unspent_outputs: HashMap<&str,Vec<u64>> = HashMap::new();
        let accumulated: u64 = 0;
        let db = self.blockchain.db.clone();
        
        


        return (0, unspent_outputs);

    }

    pub fn find_utxo(&self, pubkey_hash: Vec<u8>) -> Vec<TxOutput>{
        Vec::new()
    }

    pub fn update(&mut self, block: &Block) -> Result<(),Box<dyn Error>>{
        let db = self.blockchain.db.clone();
        let tx = db.tx(true).unwrap();
        match tx.get_bucket(UTXO_BUCKET) {
            Ok(bucket) => {
                for tx in block.transactions.clone(){
                    if !tx.is_coinbase(){
                        for vin in tx.vin{
                            let mut updated_outs = TxOutputs{outputs: Vec::new()};
                            if let Some(data) = bucket.get(vin.txid.clone()) {
                                let outs: TxOutputs = rmp_serde::from_slice(data.kv().value())?;
                                for (out_idx,out) in outs.outputs.into_iter().enumerate(){
                                if out_idx != vin.vout.into(){
                                    updated_outs.outputs.push(out);
                                }
                            }
                            if updated_outs.outputs.len() == 0{
                                bucket.delete(vin.txid);
                            }else{
                                let updated_outs_bytes = rmp_serde::to_vec(&updated_outs)
                                    .map_err(|e| Box::new(e) as Box<dyn Error>)?;
                                bucket.put(vin.txid, updated_outs_bytes);
                            }                               
                            } else {
                                println!("Nothing was found");
                            }
                        }
                    }
                    let mut new_outputs = TxOutputs{outputs: Vec::new()};
                    for out in tx.vout{
                        new_outputs.outputs.push(out);
                    }
                    let new_outputs_bytes = rmp_serde::to_vec(&new_outputs)
                                    .map_err(|e| Box::new(e) as Box<dyn Error>)?;
                    bucket.put(tx.id, new_outputs_bytes);
                }
                
            },
            Err(_) => {
                println!("Error: Bucket not found");
            }
        };
        tx.commit();
        Ok(())
    }
}

