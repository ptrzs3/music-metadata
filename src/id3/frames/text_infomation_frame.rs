use std::fmt::Display;

use super::common::Tape;

pub struct TextInfomationFrame {
    identifier: String,
    data: String,
}

impl TextInfomationFrame {
    pub fn new(identifier: String, data: String) -> Self {
        TextInfomationFrame { identifier, data }
    }
}

impl Display for TextInfomationFrame {
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

impl Tape for TextInfomationFrame {
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