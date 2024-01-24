#[derive(Debug)]
pub struct BlockSeekTable {
    pub seekpoints: Vec<SeekPoint>,
}
impl Default for BlockSeekTable {
    fn default() -> Self {
        BlockSeekTable { seekpoints: Vec::default() }
    }
}
#[derive(Debug)]
pub struct SeekPoint {
    pub sample_number_of_first_sample: u64,
    pub offset: u64,
    pub number_of_samples: u16,
}

impl Default for SeekPoint {
    fn default() -> Self {
        SeekPoint {
            sample_number_of_first_sample: u64::default(),
            offset: u64::default(),
            number_of_samples: u16::default(),
        }
    }
}
