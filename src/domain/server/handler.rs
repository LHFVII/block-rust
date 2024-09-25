use std::net::TcpListener;

pub fn start_server(node_id: String, miner_address: String) -> std::io::Result<()>{
    let mining_address = miner_address;
    let listener = TcpListener::bind("127.0.0.1:80")?;
    
    for stream in listener.incoming() {
        println!("handling...");
        //handle_client(stream?);
    }
    Ok(())
}