use crate::domain::Block;
use jammdb::{DB, Error};
const BLOCKS_BUCKET: &str = "blocks";

pub struct Blockchain {
    pub tip: Vec<u8>,
    pub db: DB
}

impl Blockchain {
    pub fn new() -> Result<Self, Error> {
        let db = DB::open("blockchain.db")?;
        let tip = {
            let tx = db.tx(true)?;
    
            let result = match tx.get_bucket(BLOCKS_BUCKET) {
                Ok(bucket) => {
                    bucket.get(b"l")
                        .map(|data| data.kv().value().to_vec())
                        .unwrap_or_else(Vec::new)
                },
                Err(_) => {
                    let block_bucket = tx.create_bucket(BLOCKS_BUCKET)?;
                    let genesis = Block::new(b"Genesis Block".to_vec(), Vec::new());
                    let genesis_hash = genesis.hash.clone();
                    
                    let block_bytes = rmp_serde::to_vec(&genesis)
                        .map_err(|_| Error::IncompatibleValue)?;
                    
                    block_bucket.put(genesis.hash, block_bytes)?;
                    block_bucket.put(b"l".to_vec(), genesis_hash.clone())?;
                    
                    genesis_hash
                }
            };
    
            tx.commit()?;
            result
        };
        
        Ok(Blockchain { 
            tip: tip,  
            db 
        })
    }

    pub fn add_block(&mut self, data: Vec<u8>) -> Result<bool, Error> {
        let tx = self.db.tx(false)?;
        let bucket = tx.get_bucket(BLOCKS_BUCKET)?;
        println!("{:?}", self.tip);
        let new_block = Block::new(data, self.tip.clone());
        let block_bytes = rmp_serde::to_vec(&new_block)
            .map_err(|_| Error::IncompatibleValue)?;
        
        bucket.put(new_block.hash.clone(), block_bytes)?;
        self.tip = new_block.hash;
        Ok(true)
    }

    
}

pub struct BlockchainIterator<'a> {
    pub current_hash: Vec<u8>,
    pub blockchain: &'a Blockchain,
}

impl<'a> Iterator for BlockchainIterator<'a> {
    
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_hash.is_empty() {
            return None;
        }

        let tx = self.blockchain.db.tx(false).ok()?;
        let bucket = tx.get_bucket(BLOCKS_BUCKET).ok()?;

        if let Some(data) = bucket.get(&self.current_hash) {
            let block: Block = rmp_serde::from_slice(data.kv().value()).ok()?;
            self.current_hash = block.prev_block_hash.clone();
            Some(block)
        } else {
            None
        }
    }
}

