use std::fmt::Display;

use super::common::Encoding;

pub struct TXXX {
    encoding: Encoding,
    description: String,
    data: String,
}
impl TXXX {
    pub fn new(encoding: Encoding, description: String, data: String) -> Self {
        TXXX {
            encoding,
            description,
            data,
        }
    }
}
impl Display for TXXX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
TXXX {{
    encoding: {:?},
    decription: {},
    data: {}
}}
",
            self.encoding, self.description, self.data
        )
    }
}
// impl Frame for TXXX {
//     fn get_date(&self) -> String {
//         self.data.clone()
//     }
// }
