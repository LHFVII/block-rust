use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::domain::block;
use crate::domain::validate_address;
use crate::domain::Blockchain;

pub struct Server {
    pub known_nodes: Vec<String>,
}

impl Server {
    pub fn new() -> Self {
        return Server {
            known_nodes: vec![],
        };
    }
    pub fn start_server(&mut self, _node_id: String, miner_address: String) -> std::io::Result<()> {
        let mining_address = miner_address;
        let result = TcpListener::bind("127.0.0.1:8000");
        match &result {
            Ok(_) => {
                println!("Listener created successfully")
            }
            Err(e) => {
                eprintln!("Something went wrong {:?}", e);
                return Ok(());
            }
        }
        let listener = result.unwrap();
        let mut bc: Blockchain;
        match Blockchain::create_blockchain(mining_address) {
            Ok(blockchain) => {
                bc = blockchain;
            }
            Err(e) => {
                bc = Blockchain::new().unwrap();
            }
        }
        println!("Blockchain created");
        println!("Listening...");
        for stream in listener.incoming() {
            match stream {
                Ok(conn) => self.handle_connection(conn, &bc),
                Err(e) => {
                    eprintln!("Sth went wrong {:?}", e);
                }
            }
        }
        Ok(())
    }

    pub fn handle_connection(&mut self, mut conn: TcpStream, bc: &Blockchain) {
        let mut command_buf = [0u8; 1];
        match conn.read_exact(&mut command_buf) {
            Ok(_) => {
                let command = command_buf[0];
                println!("Received command: {}", command);
                match command {
                    1 => {
                        self.handle_address(conn);
                    }
                    _ => println!("Unknown command: {}", command),
                }
            }
            Err(e) => eprintln!("Error reading from connection: {}", e),
        }
    }

    pub fn handle_address(&mut self, mut conn: TcpStream) {
        let mut buffer = Vec::new();
        match conn.read_to_end(&mut buffer) {
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
    /*
    pub fn request_blocks(&mut self) {
        println!("Requesting blocks");
        for node in &mut self.known_nodes {
            self.handle_get_blocks(node.as_str());
        }
    }
    pub fn send_get_blocks(&mut self, node: &str) {
        println!("Sending get blocks");
    }

    pub fn handle_get_blocks(&mut self, bc: &Blockchain) {
        println!("handling get blocks...");
        let mut buffer = Vec::new();
        let blocks = bc.get_block_hashes();
        self.handle_inv(address, blocks);
    }
    pub fn handle_inv(&mut self, address: &str, blocks: usize){
        let mut buffer = Vec::new();
        let items = "";
        let payload_type = "";
        println!("Received inventory with {:?} {:?}", items, payload_type );
        match payload_type {
            "block" =>{
                println!("Received block");
                let blocks_in_transit = payload.Items;
                let block_hash = payload.Items[0];
                sendGetData(payload.AddrFrom, "block", block_hash);

            let mut new_in_transit: Vec<Vec<u8>> = vec![vec![]];
            for (k,v) in blocks_in_transit{
                if bytes.Compare(v, block_hash) != 0 {
                    new_in_transit.push(block_hash);
                }
            }
            },
            "tx"=>{
                println!("Received tx");
                let mut tx_id := payload.Items[0];

            if mempool[hex.EncodeToString(txID)].ID == nil {
                sendGetData(payload.AddrFrom, "tx", txID);
            }
            }
        }*/
}
