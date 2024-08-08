use std::{path::Path, vec};
use crate::domain::Block;
use jammdb::{DB, Data, Error};
use serde::de::value;
use std::fmt::Write;

const DB_FILE: &str = "./blockchain.db";
const BLOCKS_BUCKET: &str = "blocks";

pub struct Blockchain {
    pub tip: Vec<u8>,
    pub db: DB
}

pub struct BlockchainIterator{
	pub current_hash: Vec<u8>,
	pub db: DB
}

impl Blockchain {
    pub fn new() -> Result<Self, Error> {
        let db = DB::open("blockchain.db")?;
        let tx = db.tx(true)?;
        let bucket_result = tx.get_bucket(BLOCKS_BUCKET);
        let tip = match bucket_result {
            Ok(bucket) => {
                if let Some(data) = bucket.get(b"l") {
                    let mut hash_string = String::new();
                    for byte in data.kv().value() {
                        write!(&mut hash_string, "{:02x}", byte).unwrap();
                    }
                    let hash = hash_string.into_bytes();
                    hash
                } else {
                    Vec::new()
                }                
            },
            Err(_) => {
                let block_bucket = tx.create_bucket(BLOCKS_BUCKET)?;
                let genesis_data = b"Genesis Block".to_vec();
                let genesis = Block::new(genesis_data, Vec::new());
                let new_hash = genesis.clone();
                let new_hash_two = genesis.clone();
                let block_bytes = rmp_serde::to_vec(&genesis).unwrap();
                block_bucket.put(genesis.hash, block_bytes)?;
                block_bucket.put("l", new_hash_two.hash)?;
                tx.commit()?;
                new_hash.hash
            }
        };
        let db_final = db.clone();
        Ok(Blockchain { tip, db: db_final })
    }

}

/*impl BlockchainIterator{
    pub fn  Next(&self) -> Block {
        
        self.currentHash;
    
        return block
    }
}*/