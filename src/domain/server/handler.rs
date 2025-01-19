use crate::domain::validate_address;
use crate::domain::Blockchain;
use crate::domain::Transaction;
use std::collections::HashMap;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

pub struct Server {
    pub node_address: String,
    pub mining_address: String,
    pub known_nodes: Vec<String>,
    pub blocks_in_transit: Vec<Vec<u8>>,
    pub mem_pool: HashMap<String, Transaction>,
}

impl Server {
    pub fn new(node_address: String, mining_address: String) -> Self {
        return Server {
            node_address: node_address,
            mining_address: mining_address,
            known_nodes: vec![],
            blocks_in_transit: vec![vec![]],
            mem_pool: HashMap::<String, Transaction>::new(),
        };
    }
    pub async fn start_server(
        &mut self,
        _node_id: String,
        miner_address: String,
    ) -> std::io::Result<()> {
        let mining_address = miner_address;

        let mut bc: Blockchain;
        match Blockchain::create_blockchain(mining_address) {
            Ok(blockchain) => {
                bc = blockchain;
            }
            Err(e) => {
                bc = Blockchain::new().unwrap();
            }
        }
        let listener = TcpListener::bind("127.0.0.1:8000").await?;
        loop {
            println!("listening");
            let (socket, _) = listener.accept().await?;
            self.handle_connection(socket, &mut bc).await;
        }
    }

    pub async fn handle_connection(&mut self, conn: TcpStream, bc: &mut Blockchain) {
        let mut command_buf = [0; 1024];
        let _result = conn
            .readable()
            .await
            .map_err(|_| println!("Something went wrong"));

        match conn.try_read(&mut command_buf) {
            Ok(_) => {
                let command = command_buf[0];
                println!("Received command: {}", command);
                match command {
                    1 => {
                        self.handle_address(command_buf, conn).await;
                    }
                    2 => {
                        self.request_blocks(bc);
                    }
                    3 => {
                        self.handle_get_blocks(bc);
                    }
                    _ => println!("Unknown command: {}", command),
                }
            }
            Err(e) => eprintln!("Error reading from connection: {}", e),
        }
    }

    pub async fn handle_address(&mut self, command_buf: [u8; 1024], mut conn: TcpStream) {
        let command_two = &command_buf[1..35];
        let address = match std::str::from_utf8(command_two) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        if !validate_address(address) {
            eprintln!("Invalid address");
            let _ = conn.write_all("Invalid address".as_bytes()).await;
            return;
        }
        if self.known_nodes.contains(&address.to_string()) {
            eprintln!("Address is already known");
            return;
        }
        self.known_nodes.push(address.to_string());
    }

    pub fn request_blocks(&mut self, bc: &mut Blockchain) {
        println!("Requesting blocks");
        /*for node in self.known_nodes.clone() {
            self.handle_get_blocks(node.as_str(), bc);
        }*/
    }

    pub fn handle_get_blocks(&mut self, bc: &mut Blockchain) {
        println!("handling get blocks...");
        let blocks = bc.get_block_hashes();
        println!("Blocks are: {:?}", blocks);
        //self.handle_inv_block(address, blocks);
    }

    pub fn handle_inv_block(&mut self, address: &str, blocks: Vec<String>) {
        let block_hash = blocks[0].clone().into_bytes();
        let mut new_in_transit: Vec<Vec<u8>> = vec![vec![]];
        for block in blocks {
            if block.into_bytes() < block_hash.clone() {
                new_in_transit.push(block_hash.clone());
            }
        }
    }
}
