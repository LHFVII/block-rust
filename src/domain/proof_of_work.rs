use crate::domain::block::{Block};
use num_bigint::BigInt;

const TARGET_BITS: u8 = 24;

pub struct ProofOfWork{
    pub block: &Block,
    pub target: BigInt,
}

impl ProofOfWork{
    fn new(&self, b: &Block) -> Self {
        let mut target = BigInt::from(1);
        target <<= 256 - TARGET_BITS;
        let pow = &ProofOfWork{block: b, target: target};
        return pow
    }

}
