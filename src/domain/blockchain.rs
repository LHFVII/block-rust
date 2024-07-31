use crate::domain::Block;

pub struct Blockchain {
    pub blocks: Vec<Block>
}

impl Blockchain {
    pub fn new(genesis_block: Block) -> Self {
        Blockchain { blocks: vec![genesis_block] }
    }
    
    pub fn add_block(&mut self, data: Vec<u8>) {
        let prev_block = self.blocks.last().expect("There is at least one block");
        let new_block = Block::new(data, prev_block.hash.clone());
        self.blocks.push(new_block);
    }
}