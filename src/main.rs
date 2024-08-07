mod domain;

fn main() {
    let mut bc = domain::Blockchain::new();

    /*let block1_data = b"Block 1 Data".to_vec();
    bc.add_block(block1_data);

    let block2_data = b"Block 2 Data".to_vec();
    bc.add_block(block2_data);*/

	/*for block in bc.blocks.iter() {
		println!("Prev. hash: {}", hex::encode(&block.prev_block_hash));
		println!("Data: {}", String::from_utf8_lossy(&block.data));
		println!("Hash: {}", hex::encode(&block.hash));
		let pow = domain::ProofOfWork::new(block.clone());
		println!("");
	}*/
}
