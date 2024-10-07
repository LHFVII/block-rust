use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::domain::{ProofOfWork, Transaction};

use super::MerkleTree;


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

    pub fn hash_transactions(&self) -> Vec<u8>{
        let tx_hashes: Vec<Vec<u8>> = self.transactions.iter()
        .map(|tx| tx.serialize_id())
        .collect();
        let merkle_tree = MerkleTree::new(tx_hashes.to_vec());
        return merkle_tree.root.unwrap().data;
    }
}