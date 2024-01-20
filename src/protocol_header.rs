use std::fmt::Display;

use crate::{frames::header::Byte, version::Version};

#[derive(Debug)]
pub struct ProtocolHeader {
    pub identifier: String,
    pub major_version: Version,
    pub revision: u8,
    pub flags: Byte,
    pub size: u32,
}

impl Display for ProtocolHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ProtocolHeader {{
    identifier: {},
    major_version: {},
    revision, {},
    flags: {:?},
    size: {} Bytes
}}",
            self.identifier, self.major_version, self.revision, self.flags, self.size
        )
    }
}
