use std::fmt::Display;

#[derive(Debug)]
pub enum Version {
    V3,
    V4,
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

impl Default for Version {
    fn default() -> Self {
        Version::Default
    }
}