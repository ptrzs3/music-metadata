use std::fmt::Display;

use super::common::Encoding;

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
// impl Heavy for USLT {
//     fn get_identifier(self) -> String {
//         self.identifier
//     }
//     fn get_raw_data(self) -> Vec<u8> {
//         self.data
//     }
//     fn get_addition(self) -> String {
//         format!(
//             "language:{}, description: {}",
//             self.language, self.descriptor
//         )
//     }
// }
