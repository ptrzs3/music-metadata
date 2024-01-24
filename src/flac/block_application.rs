#[derive(Debug)]
pub struct BlockApplication {
    id: u32,
    data: Vec<u8>,
}

impl BlockApplication {
    pub fn new(id: u32, data: Vec<u8>) -> Self {
        BlockApplication { id, data }
    }
}
impl Default for BlockApplication {
    fn default() -> Self {
        BlockApplication {
            id: u32::default(),
            data: Vec::default(),
        }
    }
}
