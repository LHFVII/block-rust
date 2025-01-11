use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::domain::Blockchain;

pub fn start_server(_node_id: String, miner_address: String) -> std::io::Result<()> {
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
            Ok(conn) => handle_connection(conn, &bc),
            Err(e) => {
                eprintln!("Sth went wrong {:?}", e);
            }
        }
    }
    Ok(())
}

pub fn handle_connection(mut conn: TcpStream, bc: &Blockchain) {
    let mut command_buf = [0u8; 1];
    match conn.read_exact(&mut command_buf) {
        Ok(_) => {
            let command = command_buf[0];
            println!("Received command: {}", command);
            match command {
                1 => {
                    handle_address(conn);
                }
                2 => {
                    let mut data = Vec::new();
                    if let Ok(_) = conn.read_to_end(&mut data) {
                        handle_get_blocks(data.len(), bc);
                    }
                }
                _ => println!("Unknown command: {}", command),
            }
        }
        Err(e) => eprintln!("Error reading from connection: {}", e),
    }
}

pub fn handle_address(mut conn: TcpStream) {
    let mut buffer = Vec::new();
    match conn.read_to_end(&mut buffer) {
        Ok(_) => {
            if let Ok(address) = String::from_utf8(buffer) {
                println!("handling address: {:?}...", address)
            } else {
                eprintln!("Invalid UTF-8 in address");
            }
        }
        Err(e) => eprintln!("Error reading address data: {}", e),
    }
}

pub fn handle_get_blocks(_request: usize, _bc: &Blockchain) {
    println!("handling address...")
}
