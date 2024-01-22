struct Header {
  is_last: bool,
  block_type: BlockType,
  length: u32
}
enum BlockType {
  STREAMINFO,
  PADDING,
  APPLICATION,
  SEEKTABLE,
  VORBISCOMMENT,
  CUESHEET,
  PICTURE,
  RESERVED,
  INVALID
}