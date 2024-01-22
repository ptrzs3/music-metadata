use std::fmt::Display;

#[derive(Debug)]
#[derive(Default)]
pub enum Version {
    V3,
    V4,
    #[default]
    Default
}
impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ver = match self {
            Version::V3 => 3,
            Version::V4 => 4,
            Version::Default => -1
        };
        write!(f, "{ver}")
    }
}
impl Clone for Version {
    fn clone(&self) -> Self {
        match self {
            Version::V3 => Version::V3,
            Version::V4 => Version::V4,
            Version::Default => Version::Default
        }
    }
}

