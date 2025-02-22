use crate::domain::Block;
use crate::domain::Blockchain;
use crate::domain::Transaction;
use std::thread;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

pub enum NodeMessage {
    Stop,
    Restart,
    TxReceived { tx: Transaction },
}

pub struct Server {
    pub node_address: String,
    pub mining_address: String,
    pub known_nodes: Vec<String>,
    pub blocks_in_transit: Vec<Vec<u8>>,
}

impl Server {
    pub fn new(node_address: String, mining_address: String) -> Self {
        return Server {
            node_address: node_address,
            mining_address: mining_address,
            known_nodes: vec![],
            blocks_in_transit: vec![vec![]],
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
            let (mut socket, _) = listener.accept().await?;
            let tx = tx.clone();
            tokio::spawn(async move {
                let mut command_buf: [u8; 1024] = [0; 1024];
                match socket.try_read(&mut command_buf) {
                    Ok(_) => {
                        let command = command_buf[0];
                        println!("Received command: {}", command);
                        match command {
                            1 => {
                                println!("Transaction received");
                                let mut buffer = Vec::new();
                                match Transaction::from_tcp_stream(&mut socket, &mut buffer).await {
                                    Ok(transaction) => {
                                        tx.send(NodeMessage::TxReceived { tx: transaction }).await;
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to read transaction: {}", e);
                                    }
                                }
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
            });
        }
    }
}

pub async fn start_mining_thread(mut rx: mpsc::Receiver<NodeMessage>) {
    let mut mem_pool: Vec<Transaction> = Vec::new();
    let mut is_mining = false;
    let mut counter = 0;
    loop {
        match rx.try_recv() {
            Ok(message) => match message {
                NodeMessage::Stop => {
                    is_mining = false;
                }
                NodeMessage::Restart => {
                    is_mining = true;
                }
                NodeMessage::TxReceived { tx } => {
                    mem_pool.push(tx);
                    if mem_pool.len() > 3 {
                        is_mining = true;
                    } else {
                        is_mining = false;
                    }
                }
            },
            Err(mpsc::error::TryRecvError::Empty) => {
                if is_mining {
                    thread::sleep(Duration::from_millis(5000));
                    let generation_tx = create_generation_transaction("").await;
                    mem_pool.push(generation_tx);
                    let candidate_txs = mem_pool.clone();
                    mem_pool = vec![];

                    //Blockchain::mine_block(candidate_txs);
                    println!("{:?} ⛏️Mining...", counter);
                    is_mining = false;
                    counter += 1;
                }
            }
            Err(mpsc::error::TryRecvError::Disconnected) => {
                println!("Channel disconnected, stopping mining");
                break;
            }
        }
    }
}

pub async fn create_generation_transaction(node_address: &str) -> Transaction {
    let reward = 20;
    let coinbase_data = String::from("placeholder");
    let generation_transaction: Transaction =
        Transaction::new_generation_tx(node_address.to_string(), reward, coinbase_data);
    return generation_transaction;
}
