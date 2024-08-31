use secp256k1::{Secp256k1, SecretKey, PublicKey};
use p256::ecdsa::{SigningKey, VerifyingKey};
use rand_core::OsRng;
use sha2::{Sha256};
use ripemd::{Ripemd160, Ripemd320, Digest};
use bs58;
use std::collections::HashMap;

const VERSION: u8 = 0x00;
const ADDRESS_CHECKSUM_LEN: usize = 4;

pub struct Wallet {
    private_key: SecretKey,
    public_key: PublicKey,
}

pub struct Wallets {
    wallets: HashMap<String, Wallet>,
}

impl Wallet {
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(&[0xcd; 32]).expect("32 bytes, within curve order");
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);

        Wallet {
            private_key: secret_key,
            public_key,
        }
    }

    pub fn new_keypair() -> (SigningKey, Vec<u8>){
        let signing_key = SigningKey::random(&mut OsRng);
        let verifying_key = VerifyingKey::from(&signing_key);
        let public_key_bytes = verifying_key.to_encoded_point(false).as_bytes().to_vec();
        (signing_key, public_key_bytes)
    }

    pub fn get_address(&self) -> Vec<u8>{
        let pub_key_bytes = self.public_key.serialize_uncompressed().to_vec();
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