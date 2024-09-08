
use serde::{Serialize, Deserialize};
use sha2::{Sha256};
use ripemd::{Ripemd160, Digest};
use bs58;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use bincode;
use std::io::{Read, Write};
use secp256k1::{PublicKey, SecretKey, Secp256k1};
use secp256k1::rand::rngs::OsRng;


const VERSION: u8 = 0x00;
const ADDRESS_CHECKSUM_LEN: usize = 4;
const WALLET_FILE: &str = "wallet.dat";

#[derive(Serialize, Deserialize)]
pub struct Wallet {
    pub private_key: SecretKey,
    pub public_key: PublicKey,
}


impl Wallet {
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);

        Wallet {
            private_key: secret_key,
            public_key,
        }
    }

    pub fn get_address(&self) -> Vec<u8>{
        let pub_key_bytes = self.public_key.serialize().to_vec();
        let pub_key_hash = hash_pubkey(pub_key_bytes);

        let mut versioned_payload = vec![VERSION];
        versioned_payload.extend_from_slice(&pub_key_hash);

        let checksum = checksum(versioned_payload.clone());

        let mut full_payload = versioned_payload;
        full_payload.extend_from_slice(&checksum);

        bs58::encode(full_payload).into_vec()
    }
}

pub fn hash_pubkey(pubkey: Vec<u8>)-> Vec<u8>{
    let public_sha256 = Sha256::digest(pubkey);
    let public_ripemd160 = Ripemd160::digest(public_sha256);
    public_ripemd160.to_vec()
}
pub fn checksum(payload: Vec<u8>)-> Vec<u8>{
    let first_sha = Sha256::digest(payload);
    let second_sha = Sha256::digest(first_sha);
    second_sha[..ADDRESS_CHECKSUM_LEN].to_vec()
}
#[derive(Serialize, Deserialize)]
pub struct Wallets {
    wallets: HashMap<String, Wallet>,
}

impl Wallets {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut wallets = Wallets {
            wallets: HashMap::new(),
        };
        wallets.load_from_file()?;
        Ok(wallets)
    }

    pub fn create_wallet(&mut self) -> String {
        let wallet = Wallet::new();
        let address = wallet.get_address();
        let stringified_address = String::from_utf8_lossy(&address).to_string();
        println!("Stringified address is {}",stringified_address.clone());
        self.wallets.insert(stringified_address.clone(), wallet);
        stringified_address
    }

    pub fn get_addresses(&self) -> Vec<String> {
        self.wallets.keys().cloned().collect()
    }

    pub fn get_wallet(&self, address: &str) -> Option<&Wallet> {
        self.wallets.get(address)
    }

    pub fn load_from_file(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(WALLET_FILE);
        if !path.exists() {
            return Ok(());
        }
        let mut file = fs::File::open(path)?;
        
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;
        
        let loaded_wallets: Wallets = bincode::deserialize(&content)?;
        
        let mut wallets: HashMap<String, Wallet> = HashMap::new();
        for (_key,wallet) in &loaded_wallets.wallets{
            let current_wallet = Wallet{private_key:wallet.private_key, public_key: wallet.public_key};
            wallets.insert(wallet.public_key.to_string(), current_wallet);
        }
        self.wallets = wallets;

        Ok(())
    }

    pub fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = bincode::serialize(&self)?;
        let mut file = fs::File::create(WALLET_FILE)?;
        file.write_all(&content)?;
        Ok(())
    }
}
