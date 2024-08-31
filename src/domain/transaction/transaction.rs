use serde::{de::value, Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::domain::Blockchain;

use super::{TxInput, TxOutput};

const SUBSIDY:u32 = 10;

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
        let txin = TxInput{txid: Vec::new(),vout:0,script_sig:"".to_string(),pubkey: data.into_bytes() };
        let txout = TxOutput::new(SUBSIDY, to);
        let mut tx = Transaction{id: Vec::new(),vin: vec![txin], vout: vec![txout]};
        tx.hash();
        return tx
    }

    pub fn is_coinbase(&self) -> bool{
        return self.vin.len() == 1 && self.vin[0].txid.len() == 0 && self.vin[0].vout == 0
    }

    pub fn new_utxo_transaction(from: &str, to: String, amount: u32,bc: &mut Blockchain) -> Self{
        let mut inputs: Vec<TxInput> = Vec::new();
        let mut outputs: Vec<TxOutput> = Vec::new();
        
        let (acc, valid_outputs) = bc.find_spendable_outputs(&from.to_string(), amount);
        
        for (txid, outs) in valid_outputs{
            let id = hex::decode(txid).unwrap();
            for out in outs{
                let input = TxInput{txid: id.clone(), vout: out,script_sig: from.into()};
                inputs.push(input);
            }
        }
        outputs.push(TxOutput{value:amount, script_pubkey: to});
        if acc>amount{
            let value = acc - (amount as u32);
            outputs.push(TxOutput{value: acc - amount, script_pubkey: from.to_string()});
        }
        let mut tx = Transaction{id: Vec::new(), vin:inputs, vout:outputs};
        tx.hash();

        return tx
    }

    pub fn hash(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let original_id = std::mem::replace(&mut self.id, Vec::new());
        let encoded: Vec<u8> = bincode::serialize(&self)?;
        self.id = original_id;
        let hash = Sha256::digest(&encoded);
        self.id = hash.to_vec();
        Ok(())
    }
}



