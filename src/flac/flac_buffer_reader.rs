use std::{fs::File, io::{self, Read}, path::Path};

use crate::util::Buffer;

pub struct FlacBufferReader {
    file: File,
}

impl FlacBufferReader {
    pub fn new<T: AsRef<Path>>(f: T) -> io::Result<Self> {
        Ok(FlacBufferReader {
            file: File::open(f)?,
        })
    }
    pub fn read_flac_header(&mut self) -> io::Result<Buffer> {
        let mut buf = vec![0; 4];
        self.file.read_exact(&mut buf)?;
        Ok(buf)
    }
}