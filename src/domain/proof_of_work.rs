use crate::domain::block::Block;
use num_bigint::BigInt;

const TARGET_BITS: u16 = 24;
const UPPER_BOUND: u16 = 256;

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
}