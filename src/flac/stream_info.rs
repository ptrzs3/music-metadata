struct StreamInfo {
  min_block_size: u16,
  max_block_size: u16,
  min_frame_size: u32,
  max_frame_size: u32,
  sample_rate: u32,
  channel: u8,
  bit_per_sample: u8,
  total_samples: u64,
  md5: u128
}