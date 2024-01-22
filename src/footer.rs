use std::fmt::Display;

use crate::{
    protocol_header::{Flag, ProtocolHeader},
    version::Version,
};

#[derive(Debug)]
pub struct Footer {
    pub identifier: String,
    pub major_version: Version,
    pub revision: u8,
    pub flags: Flag,
    pub size: u32,
}

impl Footer {
    pub fn new(
        identifier: String,
        major_version: Version,
        revision: u8,
        flags: Flag,
        size: u32,
    ) -> Self {
        Footer {
            identifier,
            major_version,
            revision,
            flags,
            size,
        }
    }
}

impl From<ProtocolHeader> for Footer {
    fn from(value: ProtocolHeader) -> Self {
        Footer::new(
            value.identifier,
            value.major_version,
            value.revision,
            value.flags,
            value.size,
        )
    }
}

impl Default for Footer {
    fn default() -> Self {
        Footer {
            identifier: String::default(),
            major_version: Version::default(),
            revision: u8::default(),
            flags: Flag::default(),
            size: u32::default(),
        }
    }
}

impl Display for Footer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "
Footer {{
    identifier: {},
    major_version: {},
    revision: {},
    flags: {{{}    }},
    size: {} Bytes,
}}", self.identifier, self.major_version, self.revision, self.flags, self.size)
    }
}