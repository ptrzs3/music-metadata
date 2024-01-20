use std::fmt::Display;

use super::common::Encoding;

pub struct WXXX {
    encoding: Encoding,
    description: String,
    data: String,
}
impl WXXX {
    pub fn new(encoding: Encoding, description: String, data: String) -> Self {
        WXXX {
            encoding,
            description,
            data,
        }
    }
}
impl Display for WXXX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
WXXX {{
    encoding: {:?},
    description: {},
    data: {}
}}
",
            self.encoding, self.description, self.data
        )
    }
}
// impl Frame for WXXX {
//     fn get_date(&self) -> String {
//         self.data.clone()
//     }
// }
