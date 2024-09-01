use serde::{Deserialize, Serialize};
use rmp_serde::{Deserializer, Serializer};
use sha2::{Sha256,Digest};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::domain::{ProofOfWork, Transaction};


#[derive(Clone)]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Block {
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub prev_block_hash: Vec<u8>,
    pub hash: Vec<u8>,
    pub nonce: u64,
}

impl Block {
    pub fn new(transactions: Vec<Transaction>, prev_block_hash: Vec<u8>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let mut block = Block {
            timestamp,
            transactions,
            prev_block_hash,
            hash: Vec::new(),
            nonce: 0,
        };
        let pow = ProofOfWork::new(block.clone());
        let (nonce, hash) = pow.run();
        block.hash = hash.to_vec();
        block.nonce = nonce;
        block
    }

    pub fn new_genesis_block(coinbase: Transaction) -> Self{
        Self::new(vec![coinbase], Vec::new())
    }

    pub fn serialize(&self) -> Vec<u8>{
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        return encoded;
    }

    pub fn deserialize(encoded: Vec<u8>) -> Self{
        let decoded = bincode::deserialize(&encoded[..]).unwrap();
        return decoded;
    }

    pub fn hash_transactions(&self) -> Vec<u8>{
        let tx_hashes: Vec<Vec<u8>> = self.transactions.iter()
        .map(|tx| tx.id.clone())
        .collect();

        let joined_hashes: Vec<u8> = tx_hashes.concat();
        
        let tx_hash = Sha256::digest(&joined_hashes);
        tx_hash.to_vec()
    }
}