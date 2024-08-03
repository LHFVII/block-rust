mod domain;

fn main() {
	let genesis_data = b"Genesis Block".to_vec();
    let genesis_block = domain::Block::new(genesis_data, Vec::new());
    let mut bc = domain::Blockchain::new(genesis_block);

    let block1_data = b"Block 1 Data".to_vec();
    bc.add_block(block1_data);

    let block2_data = b"Block 2 Data".to_vec();
    bc.add_block(block2_data);

	for block in bc.blocks.iter() {
		println!("Prev. hash: {:?}", block.prev_block_hash);
		println!("Data: {:?}", block.data);
		println!("Hash: {:?}", block.hash);
		let pow = domain::ProofOfWork::new(block.clone());
		println!("");
	}
}
