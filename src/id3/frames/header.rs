use std::fmt;

// use crate::version::Version;
use super::super::version::Version;

use super::identifiers::IDFactory;
pub type Byte = Vec<u8>;

pub struct FrameHeader {
    pub identifier: IDFactory,
    pub size: u32,
    pub flags: Vec<Byte>,
    pub version: Version,
}
impl fmt::Display for FrameHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FrameHeader {{
    identifier: {:?},
    flags: {{
        {:?},
        {:?}
    }},
    size: {} Bytes,
}}",
            self.identifier, self.flags[0], self.flags[1], self.size
        )
    }
}
