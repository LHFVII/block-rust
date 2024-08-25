use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::Blockchain;

#[derive(Clone)]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct TxInput{
    pub txid: Vec<u8>,
    pub vout: u8,
    pub script_sig: String,

}
#[derive(Clone)]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct TxOutput{
    pub value: u32,
    pub script_pubkey: String
}

#[derive(Clone)]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Transaction{
    pub id: Vec<u8>,
    pub vin: Vec<TxInput>,
    pub vout: Vec<TxOutput>
}

impl Transaction{
    pub fn new_coinbase_tx(to: String, mut data: String) -> Self{
        if data.len() == 0 {
            data = String::from("Reward to");
        }
        let txin = TxInput::new(data);
        let txout = TxOutput{value: 10, script_pubkey: to};
        let mut tx = Transaction{id: Vec::new(),vin: vec![txin], vout: vec![txout]};
        tx.set_id();
        return tx
    }

    pub fn is_coinbase(&self) -> bool{
        return self.vin.len() == 1 && self.vin[0].txid.len() == 0 && self.vin[0].vout == 0
    }

    pub fn new_utxo_transaction(from: String, to: String, amount: u32, bc: Blockchain) -> Self{
        let mut inputs: Vec<TxInput>;
        let mut outputs: Vec<TxOutput>;
        let (acc, valid_outputs) = bc.find_spendable_outputs(&from.to_string(), amount);
        

    }

    pub fn set_id(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let original_id = std::mem::replace(&mut self.id, Vec::new());
        let encoded: Vec<u8> = bincode::serialize(&self)?;
        self.id = original_id;
        let hash = Sha256::digest(&encoded);
        self.id = hash.to_vec();
        Ok(())
    }
}

impl TxInput{
    pub fn new(data:String)-> Self{
        return TxInput{txid: Vec::new(), vout: 0, script_sig:data}
    }

    pub fn can_unlock_output_with(&self,unlocking_data: String) -> bool{
        return self.script_sig == unlocking_data
    }

}

impl TxOutput{
    pub fn new(script_pubkey:String)-> Self{
        return TxOutput{value:10, script_pubkey:script_pubkey}
    }

    pub fn can_be_unlocked_with(&self,unlocking_data: String) -> bool{
        return self.script_pubkey == unlocking_data
    }
}

