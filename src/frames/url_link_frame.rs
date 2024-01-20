use std::fmt::Display;

use super::common::Tape;

pub struct URLLinkFrame {
    identifier: String,
    data: String,
}

impl URLLinkFrame {
    pub fn new(identifier: String, data: String) -> URLLinkFrame {
        URLLinkFrame { identifier, data }
    }
}

impl Display for URLLinkFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
{} {{
    data: {},    
}}",
            self.identifier, self.data
        )
    }
}

impl Tape for URLLinkFrame {
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
