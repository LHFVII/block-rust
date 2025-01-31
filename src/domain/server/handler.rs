use crate::domain::validate_address;
use crate::domain::Blockchain;
use crate::domain::Transaction;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::sync::mpsc;

enum NodeMessage {
    Stop,
    Restart,
}

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
        let (tx, rx) = mpsc::channel(10);
        let listener = TcpListener::bind("127.0.0.1:8000").await?;
        tokio::spawn(async move { start_mining_thread(rx) });
        loop {
            println!("listening...");
            let (socket, _) = listener.accept().await?;
            let tx = tx.clone();
            tokio::spawn(async move {
                let mut command_buf: [u8; 1024] = [0; 1024];
                loop {
                    match socket.try_read(&mut command_buf) {
                        Ok(_) => {
                            let command = command_buf[0];
                            println!("Received command: {}", command);
                            match command {
                                1 => {
                                    add_transaction_to_mem_pool().await;
                                }
                                2 => {
                                    let _ = tx.send(NodeMessage::Restart).await;
                                }
                                3 => {
                                    let _ = tx.send(NodeMessage::Stop).await;
                                }
                                _ => println!("Unknown command: {}", command),
                            }
                        }
                        Err(e) => eprintln!("Error reading from connection: {}", e),
                    }
                }
            });
        }
    }

    pub fn request_blocks(&mut self, bc: &mut Blockchain) {
        println!("Requesting blocks");
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
pub async fn add_transaction_to_mem_pool() {
    todo!()
}

pub fn start_mining_thread(mut rx: mpsc::Receiver<NodeMessage>) {
    let mut is_mining = true;
    let mut counter = 0;
    loop {
        match rx.try_recv() {
            Ok(message) => match message {
                NodeMessage::Stop => {
                    println!("Mining paused");
                    is_mining = false;
                }
                NodeMessage::Restart => {
                    println!("Resuming mining...");
                    is_mining = true;
                }
            },
            Err(mpsc::error::TryRecvError::Empty) => {
                if is_mining {
                    thread::sleep(Duration::from_millis(5000));
                    println!("{:?} ⛏️Mining...", counter);
                }
            }
            Err(mpsc::error::TryRecvError::Disconnected) => {
                println!("Channel disconnected, stopping mining");
                break;
            }
        }
        counter += 1;
    }
}
pub fn print_blockchain() {
    println!("Printing blockchain")
}
pub async fn add_known_node(address: String) {
    println!("adding to known nodes");
}
pub async fn stop_mining() {}
