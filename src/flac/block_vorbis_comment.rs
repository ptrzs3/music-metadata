use std::collections::HashMap;

#[derive(Debug)]
pub struct BlockVorbisComment {
    pub encoder: String,
    pub key_hash: HashMap<String, usize>,
    pub comment: Vec<Vec<String>>
}
impl Default for BlockVorbisComment {
    fn default() -> Self {
        BlockVorbisComment { encoder: String::default(), key_hash: HashMap::default(), comment: Vec::default() }
    }
}