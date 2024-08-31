use clap::{command,Parser, Subcommand};
use crate::domain::Blockchain;
use crate::domain::ProofOfWork;


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
    GetBalance{
        address: String
    },
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
        CLI{bc: None}
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
                        Commands::GetBalance { address } => self.get_balance(address),
                        Commands::Send { from, to, amount } => self.send(from, to, amount),
                    }
                }
                Err(e) => println!("That's not a valid command! Error: {}", e),
            };
        }
    }

    fn create_blockchain(&mut self,address: String) {
        self.bc = Some(Blockchain::new(address).unwrap());
        println!("Blockchain created successfully");
    }
    
    fn get_balance(&mut self,address: String) {
        let bc = &mut self.bc.as_mut().unwrap();
        let mut balance = 0;
        let utxos = bc.find_utxo(&address.to_string());
        for out in utxos{
            balance += out.value;
        }
        println!("Balance of {}: {}", address, balance);
    }

    fn send(&mut self, from: String, to: String, amount:u32){
        let bc = &mut self.bc.as_mut().unwrap();
        let tx = Transaction::new_utxo_transaction(&from, to, amount,bc);
        let mut tx_vec = Vec::new();
        tx_vec.push(tx);
        bc.mine_block(tx_vec);
        println!("Successfully sent tx");
    }
    
    fn print_chain(&mut self) {
        println!("printing...");
        let bc = &mut self.bc.as_mut().unwrap();
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

    fn show_commands(&mut self) {
        println!(r#"COMMANDS:
    create-blockchain <address> - Adds a block containing the data input.
    get-balance <address> - Gets the balance of an address
    print-chain - Shows all blocks that belong to the current blockchain.
    send <from> <to> <amount> - Sends an amount of coins from an address to another
    "#);
    }
    
}

