#[derive(Debug)]
#[allow(dead_code)]
#[derive(Default)]
pub struct BlockStreamInfo {
    pub min_block_size: u16,
    pub max_block_size: u16,
    pub min_frame_size: u32,
    pub max_frame_size: u32,
    pub sample_rate: u32,
    pub channels: u8,
    pub bits_per_sample: u8,
    pub total_samples: u64,
    pub md5: u128,
}

impl BlockStreamInfo {
    pub fn new(
        min_block_size: u16,
        max_block_size: u16,
        min_frame_size: u32,
        max_frame_size: u32,
        sample_rate: u32,
        channels: u8,
        bits_per_sample: u8,
        total_samples: u64,
        md5: u128,
    ) -> Self {
        Self {
            min_block_size,
            max_block_size,
            min_frame_size,
            max_frame_size,
            sample_rate,
            channels,
            bits_per_sample,
            total_samples,
            md5,
        }
    }
}

