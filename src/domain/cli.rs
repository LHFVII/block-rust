use clap::{command,Parser, Subcommand};
use crate::domain::Blockchain;

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

    pub fn run(&self){
        loop {
            let mut buf = String::from("");
            
            std::io::stdin().read_line(&mut buf).expect("Couldn't parse stdin");
            let line = buf.trim();
            let args = shlex::split(line).ok_or("error: Invalid quoting").unwrap();
    
            println!("{:?}" , args);
    
            match Args::try_parse_from(args.iter()).map_err(|e| e.to_string()) {
                Ok(cli) => {
                    println!("{:?}",cli.cmd);
                    match cli.cmd {
                        Commands::PrintChain => self.print_chain(),
                        Commands::AddBlock{data} => self.add_block(data)
                   }
                }
                Err(_) => println!("That's not a valid command!")
           };
        }
    }
    
    fn add_block(&self, data: String) {
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

