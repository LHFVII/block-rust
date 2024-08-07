use std::path::Path;
use crate::domain::Block;
use jammdb::{DB, Data, Error};

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
        let db = DB::open("../../blockchain.db")?;
        let tx = db.tx(true)?;
        let bucket_result = tx.get_bucket(BLOCKS_BUCKET);
        let bucket = match bucket_result {
            Ok(f) => f,
            Err(_) => {
                let block_bucket = tx.create_bucket(BLOCKS_BUCKET)?;
                block_bucket
            }
        };
        let genesis_data = b"Genesis Block".to_vec();
        let genesis = Block::new(genesis_data, Vec::new());
        let new_hash = genesis.clone();
        let block_bytes = rmp_serde::to_vec(&genesis).unwrap();
        bucket.put(genesis.hash, block_bytes)?;
        let tip = new_hash.hash;
        tx.commit()?;
        print!("Tx commited");
        Ok(Blockchain { tip, db: db })
    }

}

/*impl BlockchainIterator{
    pub fn  Next(&self) -> Block {
        
        self.currentHash;
    
        return block
    }
}*/