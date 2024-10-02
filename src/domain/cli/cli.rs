use std::default;

use clap::{command,Parser, Subcommand};
use crate::domain::Blockchain;
use crate::domain::ProofOfWork;
use crate::domain::Transaction;
use crate::domain::UTXOSet;
use crate::domain::Wallet;
use crate::domain::Wallets;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    PrintChain,
    CreateBlockchain {
        address: String,
    },
    CreateWallet,
    GetBalance{
        address: String
    },
    ListAddresses,
    Reindex,
    Send{
        from: String,
        to: String,
        amount: u32,
    }
}
pub struct CLI {
    pub bc: Option<Blockchain>,
}

impl CLI{
    pub fn new() -> Self{
        let bc = Blockchain::new();
        let mut final_bc;
        match bc{
            Ok(blockchain) => final_bc = Some(blockchain),
            Err(e) => {
                println!("No blockchain created, run the create-blockchain command first!");
                final_bc = None
            }
        }
        CLI{bc: final_bc}
    }

    pub fn run(&mut self) {
        loop {
            self.show_commands();
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).expect("Couldn't parse stdin");
            let line = buf.trim();
            let mut args = vec!["program".to_string()]; // Add a dummy program name
            args.extend(shlex::split(line).ok_or("error: Invalid quoting").unwrap());
            match Args::try_parse_from(args) {
                Ok(cli) => {
                    match cli.cmd {
                        Commands::PrintChain => self.print_chain(),
                        Commands::CreateBlockchain { address } => self.create_blockchain(address),
                        Commands::CreateWallet => self.create_wallet(),
                        Commands::GetBalance { address } => self.get_balance(address),
                        Commands::ListAddresses => self.list_addresses(),
                        Commands::Reindex => self.reindex_utxo(),
                        Commands::Send { from, to, amount } => self.send(from, to, amount),
                    }
                }
                Err(e) => println!("That's not a valid command! Error: {}", e),
            };
        }
    }

    fn create_blockchain(&mut self, address: String) {
        match Blockchain::create_blockchain(address) {
            Ok(blockchain) => {
                self.bc = Some(blockchain);
                println!("Blockchain created successfully.");
                let mut utxo_set = UTXOSet{blockchain: self.bc.as_ref().unwrap().clone()};
                match utxo_set.reindex(){
                    Ok(_) => println!("Done!"),
                    Err(e) => eprintln!("Error reindexing the utxo set: {}", e)
                }
            },
            Err(e) => {
                eprintln!("Failed to create blockchain: {}", e);
                eprintln!("Please re-run the create-blockchain command.");
            }
        }
    }

    fn create_wallet(&self){
        let mut wallets = Wallets::new().unwrap();
        let address = wallets.create_wallet();
        let _ = wallets.save_to_file();
        println!("Address: {:?}", address);
    }
    
    fn get_balance(&mut self, address: String) {
        match self.bc {
            Some(_) => {
                let utxo_set = UTXOSet{blockchain: self.bc.as_ref().unwrap().clone()};
                let decoded = bs58::decode(address.clone()).into_vec().unwrap();
                let pubkey_hash = decoded[1..decoded.len() - 4].to_vec();
                let utxos = utxo_set.find_utxo(pubkey_hash).unwrap();
                let balance: u32 = utxos.iter().map(|out| out.value).sum();
                println!("Balance of {}: {}", address, balance);
            }
            None => eprintln!("Error: Blockchain not initialized. Please create or load a blockchain first."),
        }
    }

    fn list_addresses(&self){
        let wallets = Wallets::new().unwrap();
        for wallet in wallets.wallets.into_keys(){
            println!("Wallet {}", wallet);
        }
    }

    fn send(&mut self, from: String, to: String, amount:u32){
        match self.bc {
            Some(_) => {
                let bc = self.bc.as_mut().unwrap();
                let mut utxo_set = UTXOSet{blockchain: bc.clone()};
                // Check this part later
                let wallet = Wallet::new();
                let tx = Transaction::new_utxo_transaction(wallet, to, amount, utxo_set.clone()).unwrap();
                let cbtx = Transaction::new_coinbase_tx(from,"".to_string());
                let tx_vec = vec![cbtx,tx];
                match bc.mine_block(tx_vec){
                    Ok(block) => {
                        println!("Successfully sent tx");
                        match utxo_set.update(&block){
                            Ok(_) => println!("Success"),
                            Err(e) => eprintln!("Error calculating balance: {}", e),
                        }
                    },
                    Err(e) => eprintln!("Error calculating balance: {}", e),
                }
            }
            None => eprintln!("Error: Blockchain not initialized. Please create or load a blockchain first."),
        }
    }
    
    fn print_chain(&mut self) {
        match self.bc {
            Some(_) => {
                let bc = self.bc.as_mut().unwrap();
                let mut current_block = bc.next();
                while let Some(block) = current_block {
                    println!("Prev. hash: {}", hex::encode(&block.prev_block_hash));
                    println!("Hash: {}", hex::encode(&block.hash));
                    ProofOfWork::new(block.clone());
                    println!();
                    if block.prev_block_hash.is_empty() {
                        break;
                    }
                    current_block = bc.next();
                }
            }
            None => eprintln!("Error: Blockchain not initialized. Please create or load a blockchain first."),
        }
    }

    fn reindex_utxo(&self){
        let mut utxo_set = UTXOSet{blockchain: self.bc.as_ref().unwrap().clone()};
        match utxo_set.reindex(){
            Ok(_) =>{
                let count = utxo_set.count_transactions().unwrap();
                println!("Done! There are {count} transactions in the UTXO set.")
            },
            Err(e) => eprintln!("Error reindexing the utxo set: {}", e),
        }
    }

    fn show_commands(&mut self) {
        println!(r#"COMMANDS:
    1) create-blockchain -address ADDRESS - Create a blockchain and send genesis block reward to ADDRESS
    2) create-wallet - creates a wallet and saves it into the wallets file. Returns the address.
    3) get-balance <address> - Gets the balance of an address
    4) list-addresses - Lists all available addresses
    5) print-chain - Shows all blocks that belong to the current blockchain.
    6) reindex - Rebuild the UTXO set
    7) send <from> <to> <amount> - Sends an amount of coins from an address to another
    "#);
    }
    
}

