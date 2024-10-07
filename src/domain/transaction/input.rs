use serde::{Deserialize, Serialize};

#[derive(Clone)]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct TxInput{
    pub txid: Vec<u8>,
    pub vout: u8,
    pub signature: Option<Vec<u8>>,
    pub pubkey: Option<Vec<u8>>

}

