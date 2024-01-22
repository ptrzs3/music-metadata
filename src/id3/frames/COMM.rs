use std::fmt::Display;

use super::common::{Encoding, Tape};

#[allow(dead_code)]
#[derive(Debug)]
pub struct COMM {
    identifier: String,
    encoding: Encoding,
    language: String,
    description: String,
    data: String,
}

impl COMM {
    pub fn new(encoding: Encoding, language: String, description: String, data: String) -> Self {
        COMM {
            identifier: "COMM".to_string(),
            encoding,
            language,
            description,
            data,
        }
    }
}

impl Display for COMM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
COMM {{
    encoding: {:?},
    language: {:?},
    descriptor: {},
    data: {:?}    
}}",
            self.encoding, self.language, self.description, self.data
        )
    }
}

impl Tape for COMM {
    fn identifier(&self) -> String {
        self.identifier.clone()
    }
    fn message(&self) -> String {
        self.description.clone()
    }
    fn raw(&self) -> Vec<u8> {
        Vec::default()
    }
}