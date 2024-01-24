use std::collections::HashMap;

#[derive(Debug)]
#[derive(Default)]
pub struct BlockVorbisComment {
    pub encoder: String,
    pub hm: HashMap<String, usize>,
    pub comment: Vec<Vec<String>>
}
