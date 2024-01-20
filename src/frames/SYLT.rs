use std::fmt::Display;

use super::common::Encoding;

#[allow(dead_code)]
#[derive(Debug)]
pub struct SYLT {
    identifier: String,
    encoding: Encoding,
    language: String,
    timestamp_format: u8,
    ctype: u8,
    description: String,
    data: Vec<u8>,
}

impl SYLT {
    pub fn new(
        encoding: Encoding,
        language: String,
        timestamp_format: u8,
        ctype: u8,
        description: String,
        data: Vec<u8>,
    ) -> SYLT {
        SYLT {
            identifier: "SYLT".to_string(),
            encoding,
            language,
            timestamp_format,
            ctype,
            description,
            data,
        }
    }
}

impl Display for SYLT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
SYLT {{
    encoding: {:?},
    language: {},
    timestamp_format: {},
    content_type: {},
    description: {},
    data: {:?}
}}",
            self.encoding,
            self.language,
            self.timestamp_format,
            self.ctype,
            self.description,
            self.data
        )
    }
}
