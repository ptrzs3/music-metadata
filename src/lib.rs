// 如果frame有description不定长字段，如果frame中为空，则设置为String::from("null")
mod flac;
mod id3;
mod util;

use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::File;
use std::path::Path;
use std::{fs, io};

use flac::error::FlacError;
use flac::flac_buffer_reader::FlacBufferReader;
use id3::core::{
    parse_extended_header, parse_footer_buffer, parse_frame_header, parse_frame_payload,
    parse_protocol_header,
};
use id3::error::header_error::HeaderError;
use id3::extended_header::ExtendedHeader;
use id3::footer::Footer;
use id3::frames::common::Tape;
use id3::id3_buffer_reader::ID3BufferReader;
use id3::id3v1_tag::ID3v1;
use id3::protocol_header::ProtocolHeader;
use util::Buffer;

pub struct ID3Parser<T>
where
    T: AsRef<Path>,
{
    fp: T,
    hm: HashMap<String, usize>,
    /// Some frames appear more than once
    frames: Vec<Vec<Box<dyn Tape>>>,
    /// protocol header
    pub pheader: ProtocolHeader,
    /// extended header
    pub eheader: ExtendedHeader,
    /// sum of extended header (including payload), frames, padding
    pub padding_size: u32,

    pub footer: Footer,
    /// ID3v1 tag
    pub id3v1: ID3v1,
    /// file size, for locating ID3v1
    file_size: u64,
}

impl<T> ID3Parser<T>
where
    T: AsRef<Path>,
{
    /// Create a new parser.
    pub fn new(fp: T) -> io::Result<Self> {
        let file_size = File::open(&fp)?.metadata()?.len();
        Ok(ID3Parser {
            fp,
            hm: HashMap::default(),
            frames: Vec::default(),
            pheader: ProtocolHeader::default(),
            eheader: ExtendedHeader::default(),
            padding_size: u32::default(),
            footer: Footer::default(),
            id3v1: ID3v1::default(),
            file_size,
        })
    }

    /// Return frame content that after decoding.
    ///
    /// All text information frames should call this method, including TXXX.
    ///
    /// This method is case insensitive.
    pub fn get(&self, query: &str) -> Option<Vec<String>> {
        let upper_query = query.to_uppercase();
        if let Some(index) = self.hm.get(&upper_query) {
            let mut rst = Vec::default();
            for d in self.frames[*index].iter() {
                rst.push(d.message());
            }
            Some(rst)
        } else {
            None
        }
    }

    /// Return raw data without decoding.
    ///
    /// APIC should call this method, as should SYLT.
    ///
    /// SYLT may call the `get` method in the future.
    ///
    /// This method is case insensitive.
    pub fn get_raw(&self, query: &str) -> Option<Vec<Vec<u8>>> {
        let upper_query = query.to_uppercase();
        if let Some(index) = self.hm.get(&upper_query) {
            let mut rst = Vec::default();
            for d in self.frames[*index].iter() {
                rst.push(d.raw());
            }
            Some(rst)
        } else {
            None
        }
    }

    /// Push a frame to self.frames.
    fn push(&mut self, v: Box<dyn Tape>) -> io::Result<()> {
        if let Some(index) = self.hm.get(&v.identifier()) {
            self.frames[*index].push(v);
        } else {
            let index = self.frames.len();
            self.hm.insert(v.identifier(), index);
            self.frames.push(Vec::default());
            self.frames[index].push(v);
        }
        Ok(())
    }

    /// Start parse id3v1.
    ///
    /// It is not recommended to call this method,
    ///
    /// thinking that the ID3 protocol contains very little information,
    ///
    /// unless a very old song.
    pub fn parse_id3v1(&mut self) -> io::Result<()> {
        let position = self.file_size - 128;
        let mut buffer_reader = ID3BufferReader::new(&self.fp)?;
        buffer_reader.seek(position)?;
        let buffer = buffer_reader.read_id3v1_buffer()?;
        let mut start: usize = 0;
        let header: Vec<u8> = (buffer[start..start + 3]).to_vec();
        start += 3;
        let title: Vec<u8> = (buffer[start..start + 30]).to_vec();
        start += 30;
        let artist: Vec<u8> = (buffer[start..start + 30]).to_vec();
        start += 30;
        let album: Vec<u8> = (buffer[start..start + 30]).to_vec();
        start += 30;
        let year: Vec<u8> = (buffer[start..start + 4]).to_vec();
        start += 4;
        let comment: Vec<u8> = (buffer[start..start + 30]).to_vec();
        start += 30;
        let genre: u8 = buffer[start];
        self.id3v1 = ID3v1::new(header, title, artist, album, year, comment, genre);
        Ok(())
    }

    /// Start parse id3v2.
    pub fn parse_id3v2(&mut self) -> io::Result<()> {
        let mut buffer_reader = ID3BufferReader::new(&self.fp)?;

        let mut buffer: Buffer;

        buffer = buffer_reader.read_protocol_header_buffer()?;
        let rst = parse_protocol_header(&buffer);
        if rst.is_err() {
            println!("not include ID3v2.3 or ID3v2.4");
            return Ok(());
        }
        self.pheader = rst.unwrap();
        let mut start: u32 = 0;
        if self.pheader.flags.ExtendedHeader {
            buffer = buffer_reader.read_extended_header_buffer()?;
            let mut ext = parse_extended_header(&buffer, &self.pheader.major_version);
            ext.payload = buffer_reader.skip(ext.len.into())?;
            self.eheader = ext;
            start += 10 + self.eheader.len as u32;
        }

        while start < self.pheader.size {
            buffer = buffer_reader.read_frame_header_buffer()?;
            match parse_frame_header(&buffer, &self.pheader.major_version) {
                Ok(v) => {
                    buffer = buffer_reader.read_frame_payload_buffer(v.size)?;
                    match parse_frame_payload(&buffer, &v) {
                        Ok(v) => {
                            self.push(v)?;
                        }
                        Err(e) => println!("{:?}", e),
                    }
                    start += 10 + v.size;
                }
                Err(e) => match e {
                    HeaderError::IsPadding => {
                        self.padding_size = self.pheader.size - start;
                        if self.pheader.flags.Footer {
                            // 将reader的指针定位到footer第一个字节
                            buffer_reader.seek(10 + self.pheader.size as u64)?;
                            buffer = buffer_reader.read_footer_buffer()?;
                            self.footer = parse_footer_buffer(&buffer).unwrap();
                        }
                        return Ok(());
                    }
                    HeaderError::Unimplement(id, skip) => {
                        let buf = buffer_reader.skip(skip)?;
                        start += 10 + skip;
                        println!(
                            "unimplement: {{
identifier: {},
raw: {:?}",
                            id, buf
                        );
                    }
                    HeaderError::UnknownError(s) => {
                        println!("{s}");
                        println!("The parser is stopped");
                        return Ok(());
                    }
                },
            }
        }
        Ok(())
    }

    /// As the method says.
    ///
    /// In addition, its own data will be cleared.
    pub fn change_target(&mut self, new_fp: T) {
        self.fp = new_fp;
        self.hm.clear();
        self.frames.clear()
    }

    /// Write APIC frame's raw to the current directory named with filename.jpg like 云烟成雨.jpg if there is only one APIC frame.
    ///
    /// Unless, add a underline followd by a number after the filename start with the second one, like 云烟成雨_1.jpg.
    pub fn write_image(&self) -> io::Result<()> {
        let mut t = self.fp.as_ref().to_owned();
        t.set_extension("");
        if let Some(index) = self.hm.get("APIC") {
            for (index, d) in self.frames[*index].iter().enumerate() {
                let mut fname: OsString = OsString::from(&t);
                if index > 0 {
                    fname.push("_");
                    fname.push(index.to_string());
                }
                fname.push(".jpg");
                fs::write(fname, d.raw())?
            }
        } else {
            println!("NO APIC");
        }
        Ok(())
    }
}

pub struct FlacParser<T>
where
    T: AsRef<Path>,
{
    fp: T,
}

#[allow(dead_code)]
#[allow(unused_assignments)]
#[allow(unused_variables)]
impl<T> FlacParser<T>
where
    T: AsRef<Path>,
{
    pub fn new(fp: T) -> io::Result<Self> {
        Ok(FlacParser { fp })
    }
    fn parse(&mut self) -> io::Result<()> {
        let mut buffer_reader = FlacBufferReader::new(&self.fp)?;
        let buffer: Buffer;
        buffer = buffer_reader.read_flac_header()?;
        if let Err(_) = self.parse_flac_header(&buffer) {
            println!("not include flac header");
            return  Ok(());
        }
        Ok(())
    }

    fn parse_flac_header(&mut self, buffer: &Vec<u8>) -> Result<(), FlacError> {
        if buffer[..] == [0x66, 0x4C, 0x61, 0x43] {
            return Ok(());
        }
        Err(FlacError::WrongHeader)
    }
}
