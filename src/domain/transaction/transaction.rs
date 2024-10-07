use std::collections::HashMap;
use secp256k1::{Message, Secp256k1,PublicKey, SecretKey};
use secp256k1::ecdsa::{Signature};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::error::Error;
use crate::domain::{hash_pubkey, Wallet};
use super::{TxInput, TxOutput, UTXOSet};

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
        let txin = TxInput{txid: Vec::new(),vout:0,signature:None,pubkey: Some(data.into_bytes()) };
        let txout = TxOutput::new(SUBSIDY, to);
        let tx = Transaction{id: Vec::new(),vin: vec![txin], vout: vec![txout]};
        tx.hash();
        return tx
    }

    pub fn is_coinbase(&self) -> bool{
        return self.vin.len() == 1 && self.vin[0].txid.len() == 0 && self.vin[0].vout == 0
    }

    pub fn new_utxo_transaction(wallet: &Wallet, to: String, amount: u32, mut utxo_set: UTXOSet) -> Result<Transaction, Box<dyn Error>>{
        let mut inputs: Vec<TxInput> = Vec::new();
        let mut outputs: Vec<TxOutput> = Vec::new();
        let pubkey_hash = hash_pubkey(wallet.public_key.to_string().into_bytes());
        let (acc, valid_outputs) = utxo_set.find_spendable_outputs(pubkey_hash, amount).unwrap();

        if acc < amount {
            return Err("ERROR: Not enough funds".into());
        }
        
        for (txid, outs) in valid_outputs {
            let tx_id = hex::decode(txid)?;
            for out in 1..outs {
                let input = TxInput {
                    txid: tx_id.clone(),
                    vout: out as u8,
                    signature: None,
                    pubkey: Some(wallet.public_key.clone().to_string().into_bytes()),
                };
                inputs.push(input);
            }
        }
        let from_address = format!("{:?}",wallet.get_address());
        outputs.push(TxOutput::new(amount, to));
        if acc > amount {
            outputs.push(TxOutput::new(acc - amount, from_address));
        }

        let mut tx = Transaction {
            id: Vec::new(),
            vin: inputs,
            vout: outputs,
        };

        tx.id = tx.hash();
        utxo_set.blockchain.sign_transaction(tx.clone(), &wallet.private_key);

        Ok(tx)
    }

    fn hash(&self) -> Vec<u8> {
        let mut tx_copy = self.clone();
        tx_copy.id = Vec::new();
        let serialized = tx_copy.serialize_id();
        let mut hasher = Sha256::new();
        hasher.update(&serialized);
        hasher.finalize().to_vec()
    }

    pub fn serialize_id(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(&self.id);
        result
    }

    fn trimmed_copy(&self) -> Transaction {
        let inputs: Vec<TxInput> = self.vin.iter().map(|vin| {
            TxInput {
                txid: vin.txid.clone(),
                vout: vin.vout,
                signature: None,
                pubkey: None,
            }
        }).collect();

        let outputs: Vec<TxOutput> = self.vout.iter().map(|vout| {
            TxOutput {
                value: vout.value,
                pubkey_hash: vout.pubkey_hash.clone(),
            }
        }).collect();

        Transaction {
            id: self.id.clone(),
            vin: inputs,
            vout: outputs,
        }
    }

    pub fn sign(&mut self, private_key: &SecretKey, prev_txs: HashMap<String,Transaction>){
        if self.is_coinbase(){
            return;
        }
        for vin in &self.vin{
            if !prev_txs.contains_key(&hex::encode(vin.txid.clone())){
                panic!("ERROR: previous transaction is not correct")
            }
        }
        let mut tx_copy = self.trimmed_copy();
        let secp = Secp256k1::new();

        for in_id in 0..tx_copy.vin.len() {
            let prev_tx = prev_txs.get(&hex::encode(&self.vin[in_id].txid)).unwrap();
            tx_copy.vin[in_id].signature = None;
            tx_copy.vin[in_id].pubkey = Some(prev_tx.vout[self.vin[in_id].vout as usize].pubkey_hash.clone());
            tx_copy.id = tx_copy.hash();
            tx_copy.vin[in_id].pubkey = None;

            let message = Message::from_digest_slice(&tx_copy.id).unwrap();
            let signature = secp.sign_ecdsa(&message, private_key);
            self.vin[in_id].signature = Some(signature.serialize_der().to_vec());
        }

    }

    pub fn verify(&mut self, prev_txs: HashMap<String,Transaction>) -> Result<bool, Box<dyn Error>>{
        if self.is_coinbase() {
            return Ok(true);
        }

        for vin in &self.vin {
            let prev_tx_id = hex::encode(&vin.txid);
            if !prev_txs.contains_key(&prev_tx_id) {
                return Err("ERROR: Previous transaction is not correct".into());
            }
        }

        let tx_copy = self.trimmed_copy();
        let secp = Secp256k1::verification_only();

        for (in_id, vin) in self.vin.iter().enumerate() {
            let prev_tx = &prev_txs[&hex::encode(&vin.txid)];
            let mut tx_copy = tx_copy.clone();

            tx_copy.vin[in_id].signature = Some(Vec::new());
            tx_copy.vin[in_id].pubkey = Some(prev_tx.vout[vin.vout as usize].pubkey_hash.clone());
            tx_copy.id = tx_copy.hash();
            tx_copy.vin[in_id].pubkey = Some(Vec::new());

            let message = Message::from_digest_slice(&tx_copy.id)?;
            let public_key = PublicKey::from_slice(&vin.pubkey.clone().unwrap())?;
            let signature = Signature::from_compact(&vin.signature.clone().unwrap())?;

            if !secp.verify_ecdsa(&message, &signature, &public_key).is_ok() {
                return Ok(false);
            }
        }
        Ok(true)
    }

    
}



