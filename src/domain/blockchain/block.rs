use crate::domain::{ProofOfWork, Transaction};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use super::MerkleTree;

const COIN: i64 = 100000000;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Block {
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub prev_block_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(transactions: Vec<Transaction>, prev_block_hash: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let mut block = Block {
            timestamp,
            transactions,
            prev_block_hash,
            hash: String::from(""),
            nonce: 0,
        };
        let pow = ProofOfWork::new(block.clone());
        let (nonce, hash) = pow.run();
        block.hash = hex::encode(hash);
        block.nonce = nonce;
        block
    }

    pub fn hash_transactions(&self) -> Vec<u8> {
        let tx_hashes: Vec<Vec<u8>> = self
            .transactions
            .iter()
            .map(|tx| tx.serialize_id())
            .collect();
        let merkle_tree = MerkleTree::new(tx_hashes.to_vec());
        return merkle_tree.root.unwrap().data;
    }
    pub fn get_block_value(nHeight: i64, nFees: i64) -> i64 {
        let mut n_subsidy = 50 * COIN;
        let halvings = nHeight / 4;
        if halvings >= 64 {
            return nFees;
        }
        n_subsidy >>= halvings;

        return n_subsidy + nFees;
    }
}
