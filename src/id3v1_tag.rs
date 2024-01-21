#[derive(Debug)]
#[allow(dead_code)]
pub struct ID3v1 {
    header: Vec<u8>,
    title: Vec<u8>,
    artist: Vec<u8>,
    album: Vec<u8>,
    year: Vec<u8>,
    comment: Vec<u8>,
    genre: u8,
}

impl ID3v1 {
    pub fn new(
        header: Vec<u8>,
        title: Vec<u8>,
        artist: Vec<u8>,
        album: Vec<u8>,
        year: Vec<u8>,
        comment: Vec<u8>,
        genre: u8,
    ) -> Self {
        ID3v1 {
            header,
            title,
            artist,
            album,
            year,
            comment,
            genre,
        }
    }
}

impl Default for ID3v1 {
    fn default() -> Self {
        Self {
            header: Vec::default(),
            title: Vec::default(),
            artist: Vec::default(),
            album: Vec::default(),
            year: Vec::default(),
            comment: Vec::default(),
            genre: u8::default(),
        }
    }
}
