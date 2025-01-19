use crate::domain::validate_address;
use crate::domain::Blockchain;
use crate::domain::Transaction;
use std::collections::HashMap;
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

    pub async fn handle_connection(&mut self, mut conn: TcpStream, bc: &mut Blockchain) {
        let mut command_buf = [0u8; 1];
        conn.readable().await;

        match conn.try_read(&mut command_buf) {
            Ok(_) => {
                let command = command_buf[0];
                println!("Received command: {}", command);
                match command {
                    1 => {
                        self.handle_address(conn).await;
                    }
                    2 => {
                        self.request_blocks(bc);
                    }
                    _ => println!("Unknown command: {}", command),
                }
            }
            Err(e) => eprintln!("Error reading from connection: {}", e),
        }
    }

    pub async fn handle_address(&mut self, conn: TcpStream) {
        let mut buffer = Vec::new();
        conn.readable().await;
        match conn.try_read(&mut buffer) {
            Ok(_) => {
                if let Ok(address) = String::from_utf8(buffer) {
                    println!("handling address: {:?}...", address);
                    if !validate_address(&address) {
                        eprintln!("Invalid address");
                        return;
                    }
                    if self.known_nodes.contains(&address) {
                        eprintln!("Address is already known");
                        return;
                    }
                    self.known_nodes.push(address);
                } else {
                    eprintln!("Invalid UTF-8 in address");
                }
            }
            Err(e) => eprintln!("Error reading address data: {}", e),
        }
    }

    pub fn request_blocks(&mut self, bc: &mut Blockchain) {
        println!("Requesting blocks");
        for node in self.known_nodes.clone() {
            self.handle_get_blocks(node.as_str(), bc);
        }
    }

    pub fn handle_get_blocks(&mut self, address: &str, bc: &mut Blockchain) {
        println!("handling get blocks...");
        let blocks = bc.get_block_hashes();
        self.handle_inv_block(address, blocks);
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
