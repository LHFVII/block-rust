pub struct Block<'a>{
    pub Timestamp: u64,
    pub Data: &mut [u8],
    pub PrevBlockHash: & mut [u8],
    pub Hash: &mut [u8],
    pub Nonce: u32,
}