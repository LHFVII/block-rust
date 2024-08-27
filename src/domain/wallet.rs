use secp256k1::{Secp256k1, SecretKey, PublicKey};
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
        let secret_key = SecretKey::new(&mut rand::thread_rng());
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);

        Wallet {
            private_key: secret_key,
            public_key,
        }
    }
}
