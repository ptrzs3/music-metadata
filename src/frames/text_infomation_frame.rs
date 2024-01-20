use std::fmt::Display;

// use super::common::Light;

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

// impl Light for TextInfomationFrame {
//     fn get_data(self) -> String {
//         self.data
//     }
//     fn get_identifier(self) -> String {
//         self.identifier
//     }
// }
