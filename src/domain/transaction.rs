use serde::{Deserialize, Serialize};

#[derive(Clone)]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct TxInput{
    pub txid: Vec<u8>,
    pub vout: u32,
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
        let tx = Transaction{id: Vec::new(),vin: vec![txin], vout: vec![txout]};
        return tx
    }

    pub fn set_id(){
        println!("setting");
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

