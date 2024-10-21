use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::domain::Blockchain;

pub fn start_server(node_id: String, miner_address: String) -> std::io::Result<()>{
    let mining_address = miner_address;
    let listener = TcpListener::bind("127.0.0.1:80")?;
    
    for stream in listener.incoming() {
        println!("handling...");
        //handle_client(stream?);
    }
    Ok(())
}

pub fn handle_connection(mut conn: TcpStream, bc: Blockchain){
    let buffer = &mut Vec::new();
    let request = conn.read_to_end( buffer);
    match request{
        Ok(res) =>{
            match res{
                1 => handle_address(),
                2 => handle_get_blocks(request.unwrap(), bc),
                _ => println!("Unknown command!")
            }
        }
        Err(_)=> eprintln!("")
    }
}

pub fn handle_address(){
    println!("handling address...")
}

pub fn handle_get_blocks(request: usize, bc: Blockchain){
    println!("handling address...")
}