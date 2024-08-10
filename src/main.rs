mod domain;

use domain::BlockchainIterator;

fn main() {
    let mut bc = domain::Blockchain::new().expect("Failed to create blockchain");
	println!("Blockchain created");
    let block1_data = b"Block 1 Data".to_vec();
    let _ = bc.add_block(block1_data);

    let block2_data = b"Block 2 Data".to_vec();
    let _ = bc.add_block(block2_data);
	let block_iterator = BlockchainIterator{current_hash: bc.tip.clone(), blockchain: &bc};
	for block in block_iterator {
		println!("Prev. hash: {}", hex::encode(&block.prev_block_hash));
		println!("Data: {}", String::from_utf8_lossy(&block.data));
		println!("Hash: {}", hex::encode(&block.hash));
		let pow = domain::ProofOfWork::new(block.clone());
		println!("");
	}
}
