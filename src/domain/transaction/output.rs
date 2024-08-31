use serde::{de::value, Deserialize, Serialize};

#[derive(Clone)]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct TxOutput{
    pub value: u32,
    pub pubkey_hash: Vec<u8>
}

impl TxOutput{
    pub fn new(value: u32, address: String)-> Self{
        let mut txo = TxOutput{value:value, pubkey_hash:Vec::new()};
        txo.lock(address.into_bytes());
        return txo;
    }

    pub fn is_locked_with_key(&self, pubkey_hash: Vec<u8>) -> bool{
        return self.pubkey_hash == pubkey_hash;
    }

    pub fn lock(&mut self,address: Vec<u8>){
        let mut pubkey_hash = bs58::decode(address).into_vec().ok().unwrap();
        pubkey_hash = pubkey_hash[1..=pubkey_hash.len()-4].to_vec();
        self.pubkey_hash = pubkey_hash;

    }
}

