use serde::{de::value, Deserialize, Serialize};

use crate::domain::hash_pubkey;

#[derive(Clone)]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct TxInput{
    pub txid: Vec<u8>,
    pub vout: u8,
    pub script_sig: String,
    pub pubkey: Vec<u8>

}

impl TxInput{
    pub fn uses_key(&self,pubkey_hash: Vec<u8>) -> bool{
        let locking_hash = hash_pubkey(self.pubkey.clone());
        return locking_hash == pubkey_hash;
    }

}