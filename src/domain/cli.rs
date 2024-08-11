use clap::{parser::ValueSource, Arg, Command};
use std::process;
use crate::domain::Blockchain;
pub struct CLI {
    pub bc: Blockchain,
}

impl CLI{
    pub fn new(bc: Blockchain) -> Self{
        CLI{bc: bc}
    }

    pub fn run(&self){
        let matches = Command::new("Blockchain CLI")
        .version("1.0")
        .author("Your Name")
        .about("A simple blockchain CLI")
        .subcommand(Command::new("addblock")
            .about("Add a block to the blockchain")
            .arg(Arg::new("data")
                .short('d')
                .long("data")
                .value_name("BLOCK_DATA")
                .help("Sets the data for the new block")
                .required(true)))
        .subcommand(Command::new("printchain")
            .about("Print all the blocks of the blockchain"))
        .get_matches();

    match matches.subcommand() {
        Some(("addblock", add_matches)) => {
            let data = add_matches.value_source("data").unwrap();
            self.add_block(data);
        }
        Some(("printchain", _)) => {
            self.print_chain();
        }
        _ => {
            println!("Invalid command. Use --help for usage information.");
            process::exit(1);
        }
    }

    }
    
    fn add_block(&self, data: ValueSource) {
        println!("adding block to chain...beep boop")
        /*self.bc.add_block(data);
        println!("Success!")*/
    }
    
    fn print_chain(&self) {
        println!("Printing chain...beep boop")
        /*bci := cli.bc.Iterator()
    
        for {
            block := bci.Next()
    
            fmt.Printf("Prev. hash: %x\n", block.PrevBlockHash)
            fmt.Printf("Data: %s\n", block.Data)
            fmt.Printf("Hash: %x\n", block.Hash)
            pow := NewProofOfWork(block)
            fmt.Printf("PoW: %s\n", strconv.FormatBool(pow.Validate()))
            fmt.Println()
    
            if len(block.PrevBlockHash) == 0 {
                break
            }
        }*/
    }
}

