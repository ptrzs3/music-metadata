#[derive(Debug)]
pub enum ID3Error {
    Unimplement(String, u32),
    IsPadding,
    UnknownError(String),
}
