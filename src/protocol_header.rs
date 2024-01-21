use std::fmt::Display;

use crate::version::Version;

#[derive(Debug)]
pub struct ProtocolHeader {
    pub identifier: String,
    pub major_version: Version,
    pub revision: u8,
    pub flags: Flag,
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

impl Default for ProtocolHeader {
    fn default() -> Self {
        ProtocolHeader {
            identifier: String::default(),
            major_version: Version::default(),
            revision: u8::default(),
            flags: Flag::default(),
            size: u32::default()
        }
    }
}

#[derive(Debug)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct Flag {
    Unsynchronisation: bool,
    pub ExtendedHeader: bool,
    Experimental: bool,
    Footer: bool
}

impl Flag {
    pub fn new(flag: u8) -> Flag {
        let uns = (flag & 128) >> 7 == 1;
        let ext = (flag & 64) >> 6 == 1;
        let exp = (flag & 32) >> 5 == 1;
        let foo = (flag & 16) >> 4 == 1;
        Flag {
            Unsynchronisation: uns,
            ExtendedHeader: ext,
            Experimental: exp,
            Footer: foo
        }
    }
}

impl Default for Flag {
    fn default() -> Self {
        Flag { Unsynchronisation: false, ExtendedHeader: false, Experimental: false, Footer: false }
    }
}