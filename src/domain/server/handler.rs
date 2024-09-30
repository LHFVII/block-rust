use std::net::TcpListener;

use crate::domain::Blockchain;

pub fn start_server(node_id: String, miner_address: String) -> std::io::Result<()>{
    let node_address = format!("127.0.0.1:{node_id}");
    let mining_address = miner_address;
    let listener = TcpListener::bind(node_address)?;
    let bc = Blockchain::new().unwrap();
    for stream in listener.incoming() {
        println!("handling...");
        //handle_client(stream?);
    }
    Ok(())
}