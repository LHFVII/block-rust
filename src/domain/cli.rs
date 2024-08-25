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
    AddBlock {
        data: String,
    }
}
pub struct CLI {
    pub bc: Blockchain,
}

impl CLI{
    pub fn new(bc: Blockchain) -> Self{
        CLI{bc: bc}
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
                        Commands::AddBlock { data } => self.add_block(data),
                    }
                }
                Err(e) => println!("That's not a valid command! Error: {}", e),
            };
        }
    }
    
    fn add_block(&mut self, data: String) {
        let data_vec = data.into_bytes();
        let _ = self.bc.add_block(data_vec);
        println!("Success!")
    }

    fn get_balance(&mut self,address: String) {
        let mut balance = 0;
        let utxos = self.bc.find_utxo(&address.to_string());
        for out in utxos{
            balance += out.value;
        }
        println!("Balance of {}: {}", address, balance);
    }
    
    fn print_chain(&mut self) {
        println!("printing...");
        let mut current_block = self.bc.next();
    
        while let Some(block) = current_block {
            println!("Prev. hash: {}", hex::encode(&block.prev_block_hash));
            println!("Data: {}", String::from_utf8_lossy(&block.data));
            println!("Hash: {}", hex::encode(&block.hash));
            ProofOfWork::new(block.clone());
            println!();
    
            if block.prev_block_hash.is_empty() {
                break;
            }
    
            current_block = self.bc.next();
        }
    }

    fn show_commands(&mut self) {
        println!(r#"COMMANDS:
    print-chain - Shows all blocks that belong to the current blockchain.
    add-block <data> - Adds a block containing the data input.
    "#);
    }
    
}

