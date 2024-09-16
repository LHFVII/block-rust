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

pub struct TxOutputs{
    pub outputs: Vec<TxOutput>
}

impl TxOutputs{
    pub fn serialize(&self) -> Vec<u8>{
        bincode::deserialize(self).unwrap()
        
    }
}

fn deserialize_outputs(data: Vec<u8>) -> TxOutputs {
	let encoded = bincode::deserialize(&data).unwrap();
    TxOutputs{outputs: encoded}

}

// Acts as a cache that is built from all blockchain transactions
pub struct UTXOSet{
    pub blockchain: Blockchain
}

const UTXO_BUCKET: &str = "UTXOSet";

impl UTXOSet{
    pub fn reindex(&mut self) -> Result<Self, Box<dyn Error>>{
        let db = self.blockchain.db.clone();
        let tx = db.tx(true)?;
        tx.delete_bucket(UTXO_BUCKET);
        let block_bucket = tx.create_bucket(UTXO_BUCKET)?;
        let utxo = self.blockchain.find_utxo();
        for (tx_id, outs) in utxo.into_iter().enumerate(){
            let key = decode(tx_id);
            block_bucket.put(key, outs.serialize())
        }
        tx.commit()?;
    }
    pub fn find_spendable_outputs(&self,pubkey_hash: Vec<u8>, amount: u64) -> (u64, HashMap<&str,Vec<u64>>){

    }

    pub fn find_utxo(&self, pubkey_hash: Vec<u8>) -> Vec<TxOutput>{
        Vec::new()
    }

    pub fn update(&mut self, block: &Block){
        let db = self.blockchain.db;
        let tx = db.tx(true)?;
        let result = match tx.get_bucket(UTXO_BUCKET) {
            Ok(bucket) => {
                for tx in block.transactions{
                    if !tx.is_coinbase(){
                        for vin in tx.vin{
                            let updated_outs = TxOutputs{outputs: Vec::new()};
                            let out_bytes = bucket.get(vin.txid);
                            let outs = deserialize_outputs(out_bytes);

                            for (out_idx,out) in outs.outputs.into_iter().enumerate(){
                                if out_idx != vin.vout{
                                    updated_outs.outputs.push(out);
                                }
                            }

                            if updated_outs.outputs.len() == 0{
                                bucket.delete(vin.txid);
                            }else{
                                bucket.put(vin.txid, updated_outs.serialize());

                            }
                        }
                    }
                    let new_outputs = TxOutputs{outputs: Vec::new()};
                    for out in tx.vout{
                        new_outputs.outputs.push(out);
                    }
                    bucket.put(tx.id, new_outputs.serialize());
                }
                
            },
            Err(_) => {
                println!("Error: Bucket not found");
            }
        };
        tx.commit()?;

    }
}

