use std::fmt::Display;

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
