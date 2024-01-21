use crate::version::Version;

#[derive(Debug)]
pub struct ExtendedHeader {
    pub ver: Version,
    pub len: u8,
    pub data: Vec<u8>,
    pub payload: Vec<u8>,
}
impl ExtendedHeader {
    pub fn new(ver: Version, len: u8, data: Vec<u8>) -> Self {
        ExtendedHeader {
            ver,
            len,
            data,
            payload: Vec::default(),
        }
    }
}

impl Default for ExtendedHeader {
    fn default() -> Self {
        ExtendedHeader {
            ver: Version::default(),
            len: u8::default(),
            data: Vec::default(),
            payload: Vec::default(),
        }
    }
}
