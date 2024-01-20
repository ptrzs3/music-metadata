#[derive(Debug)]
pub enum FrameError {
    Unimplement(String, u32),
    IsPadding,
    UnknownError(String),
}
