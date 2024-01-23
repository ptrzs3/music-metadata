#[derive(Debug)]
pub struct BlockHeader {
  pub is_last: bool,
  pub block_type: BlockType,
  pub length: u32
}

impl BlockHeader {
    pub fn new(is_last: bool, block_type: BlockType, length: u32) -> Self {
        BlockHeader { is_last, block_type, length }
    }
}

#[derive(Debug)]
pub enum BlockType {
  STREAMINFO,
  PADDING,
  APPLICATION,
  SEEKTABLE,
  VORBISCOMMENT,
  CUESHEET,
  PICTURE,
  INVALID
}