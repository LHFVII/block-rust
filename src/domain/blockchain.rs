use std::path::Path;
use crate::domain::Block;
use lmdb::{Cursor, RwCursor, Database,DatabaseFlags,Environment,Transaction, WriteFlags};

const DB_FILE: &str = "blockchain.db";
const BLOCKS_BUCKET: &str = "blocks";

pub struct Blockchain {
    pub tip: Vec<u8>,
    pub db: Database
}

pub struct BlockchainIterator{
	pub currentHash: Vec<u8>,
	pub db: Database
}

impl Blockchain {
    fn new() -> Result<Self, lmdb::Error> {
        let path = Path::new(DB_FILE);
        let env = Environment::new()
            .set_map_size(10485760)
            .open(path)?;

        let db: Database = env.create_db(Some(BLOCKS_BUCKET), DatabaseFlags::empty())?;

        let mut tip = Vec::new();

        let txn = env.begin_rw_txn()?;
        {
            let mut cursor = txn.open_rw_cursor(db)?;
            if let None = cursor.get(Some(b"l"), None, 1) {
                println!("No existing blockchain found. Creating a new one...");
                let genesis_data = b"Genesis Block".to_vec();
                let genesis = Block::new(genesis_data, Vec::new());
                cursor.put(genesis.hash, &genesis.serialize(), WriteFlags::empty())?;
                cursor.put(b"l", genesis.hash, WriteFlags::empty())?;
                tip = genesis.hash;
            } else {
                tip = cursor.get(Some(b"l"), None, 1)
                    .unwrap()
                    .1
                    .to_vec();
            }
        }
        txn.commit()?;

        Ok(Blockchain { tip, env })
    }

    fn add_block(&mut self, data: String) -> Result<(), lmdb::Error> {
        let mut last_hash = Vec::new();
        {
            let txn = self.env.begin_ro_txn()?;
            let db = txn.open_db(Some(BLOCKS_BUCKET))?;
            last_hash = txn.get(db, b"l")?.to_vec();
        }
        let new_block = Block::new(data, last_hash);
        let txn = self.env.begin_rw_txn()?;
        {
            let db = txn.open_db(Some(BLOCKS_BUCKET))?;
            txn.put(db, new_block.hash.as_slice(), &new_block.serialize(), WriteFlags::empty())?;
            txn.put(db, b"l", new_block.hash.as_slice(), WriteFlags::empty())?;
        }
        txn.commit()?;

        self.tip = new_block.hash;

        Ok(())
    }
}