#[derive(Debug)]
#[allow(dead_code)]
#[derive(Default)]
pub struct BlockApplication {
    id: u32,
    data: Vec<u8>,
}

impl BlockApplication {
    pub fn new(id: u32, data: Vec<u8>) -> Self {
        BlockApplication { id, data }
    }
}

