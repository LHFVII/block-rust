use std::time::{SystemTime, UNIX_EPOCH};
use crate::domain::ProofOfWork;

#[derive(Clone)]
pub struct Block {
    pub timestamp: u64,
    pub data: Vec<u8>,
    pub prev_block_hash: Vec<u8>,
    pub hash: Vec<u8>,
    pub nonce: u64,
}

impl Block {
    pub fn new(data: Vec<u8>, prev_block_hash: Vec<u8>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let mut block = Block {
            timestamp,
            data,
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
}