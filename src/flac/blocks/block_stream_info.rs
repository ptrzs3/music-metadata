#[derive(Debug)]
#[allow(dead_code)]
pub struct BlockStreamInfo {
    min_block_size: u16,
    max_block_size: u16,
    min_frame_size: u32,
    max_frame_size: u32,
    sample_rate: u32,
    channels: u8,
    bits_per_sample: u8,
    total_samples: u64,
    md5: u128,
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
impl Default for BlockStreamInfo {
    fn default() -> Self {
        BlockStreamInfo {
            min_block_size: u16::default(),
            max_block_size: u16::default(),
            min_frame_size: u32::default(),
            max_frame_size: u32::default(),
            sample_rate: u32::default(),
            channels: u8::default(),
            bits_per_sample: u8::default(),
            total_samples: u64::default(),
            md5: u128::default(),
        }
    }
}
