use std::{
    fs::File,
    io::{self, Read, Seek},
    path::Path,
};

use crate::util::Buffer;

pub struct ID3BufferReader {
    file: File,
}

impl ID3BufferReader {
    pub fn new<T: AsRef<Path>>(f: T) -> io::Result<Self> {
        Ok(ID3BufferReader {
            file: File::open(f)?,
        })
    }

    pub fn read_protocol_header_buffer(&mut self) -> io::Result<Buffer> {
        let mut buf = vec![0; 10];
        self.file.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn read_frame_header_buffer(&mut self) -> io::Result<Buffer> {
        self.read_protocol_header_buffer()
    }

    pub fn read_extended_header_buffer(&mut self) -> io::Result<Buffer> {
        self.read_protocol_header_buffer()
    }

    pub fn read_frame_payload_buffer(&mut self, length: u32) -> io::Result<Buffer> {
        let mut buf = vec![0; length as usize];
        self.file.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn read_id3v1_buffer(&mut self) -> io::Result<Buffer> {
        self.read_frame_payload_buffer(128)
    }

    pub fn read_footer_buffer(&mut self) -> io::Result<Buffer> {
        self.read_protocol_header_buffer()
    }

    pub fn skip(&mut self, length: u32) -> io::Result<Buffer> {
        // self.file.seek(io::SeekFrom::Current(length as i64))?;
        self.read_frame_payload_buffer(length)
    }

    /// absolute position from file start
    pub fn seek_to(&mut self, location: u64) -> io::Result<u64>{
        self.file.seek(io::SeekFrom::Start(location))
    }
}
