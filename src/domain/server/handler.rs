use std::{fs, net::{TcpListener, TcpStream}, path::Path};
use std::io::{Read, Write};

use crate::domain::Blockchain;

const KNOWN_NODES_FILE: &str = "known_nodes.dat";

pub fn start_server(node_id: String, miner_address: String) -> std::io::Result<()>{
    let known_nodes = KnownNodes::new();

    let node_address = format!("127.0.0.1:{node_id}");
    let mining_address = miner_address;
    let listener = TcpListener::bind(node_address.clone())?;
    let bc = Blockchain::new(node_id).unwrap();
    if node_address != known_nodes.nodes[0]{
        send_version(known_nodes.nodes[0].clone(), &bc)
    }
    for stream in listener.incoming() {
        println!("handling...");
        handle_connection(stream?, &bc.clone());
    }
    Ok(())
}

pub fn handle_connection(mut conn: TcpStream, bc: &Blockchain){
    let mut buffer = Vec::new();
    conn.read(&mut buffer).unwrap();
    let command = bytes_to_command(buffer);
    println!("Received command: {}", command);
    match command.as_str(){
       "address" => println!("sth"),
       "tx" => println!("sth"),
       &_ => println!("Command not supported..."),
    };
}

pub fn bytes_to_command(bytes: Vec<u8>) -> String{
    let mut command = Vec::new();
    for byte in bytes{
        if byte != 0x0{
            command.push(byte)
        }
    }
    return String::from_utf8(command).unwrap()
}

pub fn send_version(address: String, bc: &Blockchain){
    println!("Sending version...");
}

#[derive(serde::Serialize)]
pub struct KnownNodes {
    pub nodes: Vec<String>,
}

impl KnownNodes{
    pub fn new() -> Self{
        let path = Path::new(KNOWN_NODES_FILE);
        let mut nodes =  KnownNodes{nodes: Vec::new()};
        if !path.exists() {
            let known_nodes = vec![String::new()];
            nodes.nodes = known_nodes;
            nodes.save_to_file();
        } else {
            nodes.load_known_nodes_from_file();
        }
        return nodes;
    }
    pub fn load_known_nodes_from_file(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(KNOWN_NODES_FILE);
        if !path.exists() {
            return Ok(());
        }
        let mut file = fs::File::open(path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;
        let known_nodes: Vec<String> = bincode::deserialize(&content)?;
        self.nodes = known_nodes;
        Ok(())
    }
    
    pub fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = bincode::serialize(&self)?;
        let mut file = fs::File::create(KNOWN_NODES_FILE)?;
        file.write_all(&content)?;
        Ok(())
    }
}

