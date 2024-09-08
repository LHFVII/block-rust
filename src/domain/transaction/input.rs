use serde::{Deserialize, Serialize};

use crate::domain::hash_pubkey;

#[derive(Clone)]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct TxInput{
    pub txid: Vec<u8>,
    pub vout: u8,
    pub signature: Option<Vec<u8>>,
    pub pubkey: Option<Vec<u8>>

}

impl TxInput{
    pub fn uses_key(&self,pubkey_hash: Vec<u8>) -> bool{
        let locking_hash = hash_pubkey(self.pubkey.clone().unwrap());
        return locking_hash == pubkey_hash;
    }

}