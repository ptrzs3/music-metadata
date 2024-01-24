#[derive(Debug)]
#[derive(Default)]
pub struct BlockSeekTable {
    pub seekpoints: Vec<SeekPoint>,
}

#[derive(Debug)]
#[derive(Default)]
pub struct SeekPoint {
    pub sample_number_of_first_sample: u64,
    pub offset: u64,
    pub number_of_samples: u16,
}


