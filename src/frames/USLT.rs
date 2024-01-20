use std::fmt::Display;

use super::common::{Encoding, Tape};

#[allow(dead_code)]
#[derive(Debug)]
pub struct USLT {
    identifier: String,
    encoding: Encoding,
    language: String,
    descriptor: String,
    data: String,
}

impl USLT {
    pub fn new(encoding: Encoding, language: String, descriptor: String, data: String) -> Self {
        USLT {
            identifier: "USLT".to_string(),
            encoding,
            language,
            descriptor,
            data,
        }
    }
}

impl Display for USLT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
USLT {{
    encoding: {:?},
    language: {},
    descriptor: {},
    data: {}
}}",
            self.encoding, self.language, self.descriptor, self.data
        )
    }
}

impl Tape for USLT {
    fn identifier(&self) -> String {
        self.identifier.clone()
    }
    fn message(&self) -> String {
        self.data.clone()
    }
    fn raw(&self) -> Vec<u8> {
        Vec::default()
    }
}