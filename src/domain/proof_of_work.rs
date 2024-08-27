use crate::domain::block::Block;
use num_bigint::BigInt;
use std::cmp::Ordering;
use sha2::{Sha256,Digest};



const TARGET_BITS: u16 = 20;
const UPPER_BOUND: u16 = 256;
const MAX_NONCE: u64 = u64::MAX;

pub struct ProofOfWork {
    pub block: Block,
    pub target: BigInt,
}

impl ProofOfWork {
    pub fn new(b: Block) -> Self {
        let mut target = BigInt::from(1);
        target <<= UPPER_BOUND - TARGET_BITS;
        ProofOfWork {
            block: b,
            target,
        }
    }

    fn prepare_data(&self, nonce: u64) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&self.block.prev_block_hash);
        data.extend_from_slice(&self.block.hash_transactions());
        data.extend_from_slice(&int_to_hex(self.block.timestamp));
        data.extend_from_slice(&int_to_hex(TARGET_BITS as u64));
        data.extend_from_slice(&int_to_hex(nonce as u64));
        data
    }
    
    pub fn run(&self) -> (u64, [u8; 32]) {
        let mut hash_int = BigInt::from(0u32);
        let mut hash = [0u8; 32];
        let mut nonce = 0u64;
        
        while nonce < MAX_NONCE {
            let data = self.prepare_data(nonce);
            hash = Sha256::digest(&data).into();
            hash_int = BigInt::from_bytes_be(num_bigint::Sign::Plus, &hash);
            if hash_int.cmp(&self.target) == Ordering::Less {
                break;
            } else {
                nonce += 1;
            }
        }
        println!("\n\n");
        (nonce, hash)
    }
    
    pub fn validate(&self) -> bool {
        let data = self.prepare_data(self.block.nonce);
        let hash = Sha256::digest(&data);
        let hash_int = BigInt::from_bytes_be(num_bigint::Sign::Plus, &hash);
        hash_int.cmp(&self.target) == Ordering::Less
    }

}

fn int_to_hex(n: u64) -> Vec<u8> {
    n.to_be_bytes().to_vec()
}