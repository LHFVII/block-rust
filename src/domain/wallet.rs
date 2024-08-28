use secp256k1::{Secp256k1, SecretKey, PublicKey};
use p256::ecdsa::{SigningKey, VerifyingKey};
use rand_core::OsRng;
use std::collections::HashMap;

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
}
