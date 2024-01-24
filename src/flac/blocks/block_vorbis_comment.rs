use std::collections::HashMap;

#[derive(Debug)]
#[derive(Default)]
pub struct BlockVorbisComment {
    pub encoder: String,
    pub key_hash: HashMap<String, usize>,
    pub comment: Vec<Vec<String>>
}
