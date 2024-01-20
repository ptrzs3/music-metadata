use std::fmt::Display;

pub struct RarelyUsed {
    identifier: String,
    payload: Vec<u8>,
}

impl RarelyUsed {
    pub fn new(identifier: String, payload: Vec<u8>) -> RarelyUsed {
        RarelyUsed {
            identifier,
            payload,
        }
    }
}
impl Display for RarelyUsed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
RarelyUsed {{
    identifier:{},
    payload:{:?}
}}",
            self.identifier, self.payload
        )
    }
}
