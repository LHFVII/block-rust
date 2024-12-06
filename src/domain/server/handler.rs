use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::domain::Blockchain;

pub fn start_server(node_id: String, miner_address: String) -> std::io::Result<()>{
    let mining_address = miner_address;
    println!("Creating listener ");
    let result = TcpListener::bind("127.0.0.1:8000");
    match &result{
        Ok(_)=>{
            println!("Listener created successfully")
        },
        Err(e)=>{
            eprintln!("Something went wrong {:?}", e);
            return Ok(());
        }
    }
    let listener = result.unwrap();
    println!("Creating blockchain ");
    let bc = Blockchain::new().unwrap();
    println!("Blockchain created");
    for stream in listener.incoming() {
        println!("handling...");
        match stream{
            Ok(conn)=>{handle_connection(conn, &bc)},
            Err(e)=>{
                eprintln!("Sth went wrong {:?}", e);
            }
        }
    }
    Ok(())
}

pub fn handle_connection(mut conn: TcpStream, bc: &Blockchain){
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

pub fn handle_get_blocks(request: usize, bc: &Blockchain){
    println!("handling address...")
}