use sha2::{Sha256,Digest};

pub struct MerkleTree{
    pub root: Node
}

type Node = Option<Box<MerkleNode>>;

pub struct MerkleNode{
    pub left: Node,
    pub right: Node,
    pub data: Vec<u8>

}

impl MerkleTree{
    pub fn new(mut data: Vec<Vec<u8>>) -> Self{
        let cloned_data = data.clone();
        let mut nodes: Vec<Node> = Vec::new();
        if data.len() % 2 != 0{
            let new_data = data.get(data.len()-1).unwrap().clone();
            data.push(new_data);
        }
        for datum in data{
            let node = MerkleNode::new(None, None, datum);
            nodes.push(Some(Box::new(node)));
        }
        let mut i: usize = 0;
        while i < (cloned_data.len()/2){
            let mut new_level: Vec<Node> = Vec::new();
            let mut j = 0;
            while j < nodes.len(){
                let current_left = nodes.get_mut(j).unwrap().take();
                let current_right = nodes.get_mut(j+1).unwrap().take();
                let node = MerkleNode::new(current_left, current_right, Vec::new());
                new_level.push(Some(Box::new(node)));
                j +=2
            }
            nodes = new_level;
            i +=1;
        }
        let merkle_tree = MerkleTree{root:nodes.pop().unwrap_or(None)};
        merkle_tree
    }
}

impl MerkleNode{
    pub fn new(left: Node, right: Node, data: Vec<u8>) -> Self{
        let mut merkle_node = MerkleNode{left: None, right: None, data: Vec::new()};
        if left.is_none() && right.is_none(){
            let hash = Sha256::digest(data);
            merkle_node.data = hash[..].to_vec();
        }else{
            let mut prev_hashes = Vec::new();
            if let Some(left_node) = &left {
                prev_hashes.extend_from_slice(&left_node.data);
            }
            if let Some(right_node) = &right {
                prev_hashes.extend_from_slice(&right_node.data);
            }
            let hash = Sha256::digest(&prev_hashes);
            merkle_node.data = hash.to_vec();
        }
        merkle_node.left = left;
        merkle_node.right = right;
        return merkle_node;

    }
}