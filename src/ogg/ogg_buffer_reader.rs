use std::{fs::File, io::{self, Read, Seek}, path::Path};

use crate::util::Buffer;

pub struct OggBufferReader {
    file: File,
}
impl OggBufferReader {
    pub fn new<T: AsRef<Path>>(f: T) -> io::Result<Self> {
        Ok(OggBufferReader {
            file: File::open(f)?,
        })
    }
    pub fn read_buffer(&mut self, length: u32) -> io::Result<Buffer> {
        let mut buf = vec![0; length as usize];
        self.file.read_exact(&mut buf)?;
        Ok(buf)
    }
    pub fn read_one(&mut self) -> io::Result<u8> {
        Ok(self.read_buffer(1).unwrap()[0])
    }
    /// absolute position from file start
    // pub fn seek_to(&mut self, location: u64) -> io::Result<u64>{
    //     self.file.seek(io::SeekFrom::Start(location))
    // }
    pub fn skip(&mut self, length: u64) -> io::Result<u64> {
        self.file.seek(io::SeekFrom::Current(length as i64))
        // Ok(())
    }
}
