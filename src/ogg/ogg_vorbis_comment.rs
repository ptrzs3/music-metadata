use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
pub enum HeaderType {
    IdentifierHeader,
    CommentHeader,
    SetupHeader
}
#[derive(Debug)]
pub struct CommentHeader {
    pub header_type: HeaderType,
    pub packet_pattern: String,
    pub company_info: String,
    pub comment_body: CommentBody,
    pub end: bool
}
impl Default for CommentHeader {
    fn default() -> Self {
        CommentHeader {
            header_type: HeaderType::CommentHeader,
            packet_pattern: String::default(),
            company_info: String::default(),
            comment_body: CommentBody::default(),
            end: false
        }
    }
}
#[derive(Debug)]
pub struct CommentBody {
    pub hm: HashMap<String, usize>,
    pub comment: Vec<Vec<String>>,
}
impl Default for CommentBody {
    fn default() -> Self {
        CommentBody {
            hm: HashMap::default(),
            comment: Vec::default(),
        }
    }
}
