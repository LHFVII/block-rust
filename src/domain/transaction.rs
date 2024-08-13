pub struct TxInput{
    pub txid: Vec<u8>,
    pub vout: u32,
    pub script_sig: String,

}
pub struct TxOutput{
    pub value: u32,
    pub script_pubkey: String
}
pub struct Transaction{
    pub id: Vec<u8>,
    pub vin: Vec<TxInput>,
    pub vout: Vec<TxOutput>
}

impl Transaction{
    pub fn new_coinbase_tx(self, to: String, mut data: String) -> Self{
        if data.len() == 0 {
            data = String::from("Reward to");
        }
        let txin = TxInput{txid: Vec::new(), vout: 0, script_sig:data};
        let txout = TxOutput{value: 10, script_pubkey: to};
        let tx = Transaction{id: Vec::new(),vin: vec![txin], vout: vec![txout]};
        return tx
    }

    pub fn set_id(){
        println!("setting");
    }
}

