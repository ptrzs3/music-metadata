#[derive(Debug)]
pub enum HeaderError {
    Unimplement(String, u32),
    IsPadding,
    UnknownError(String),
}
