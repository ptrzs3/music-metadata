use std::fmt::Display;

use super::common::Encoding;

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

// impl Heavy for COMM {
//     fn get_identifier(self) -> String {
//         self.identifier
//     }
//     fn get_raw_data(self) -> Vec<u8> {
//         self.data
//     }
//     fn get_addition(self) -> String {
//         format!("language: {}, description: {}", self.language, self.description)
//     }
// }
