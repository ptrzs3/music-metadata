// 如果frame有description不定长字段，如果frame中为空，则设置为String::from("null")
mod flac;
mod id3;
mod util;

use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;

use flac::block_header::{BlockHeader, BlockType};
use flac::block_picture::PicType;
use flac::block_seektable::{SeekPoint, SeekTable};
use flac::error::FlacError;
use flac::flac_buffer_reader::FlacBufferReader;
use flac::stream_info::StreamInfo;
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

// https://xiph.org/flac/format.html#metadata_block_vorbis_comment
pub struct FlacParser<T>
where
    T: AsRef<Path>,
{
    fp: T,
    vorbis_hash: HashMap<String, usize>,
    vorbis: Vec<Vec<String>>,
}

#[allow(dead_code)]
#[allow(unused_assignments)]
#[allow(unused_variables)]
impl<T> FlacParser<T>
where
    T: AsRef<Path>,
{
    pub fn new(fp: T) -> io::Result<Self> {
        Ok(FlacParser {
            fp,
            vorbis_hash: HashMap::default(),
            vorbis: Vec::default(),
        })
    }

    pub fn parse(&mut self) -> io::Result<()> {
        let mut buffer_reader = FlacBufferReader::new(&self.fp)?;
        let mut buffer: Buffer;
        buffer = buffer_reader.read_flac_header()?;
        if let Err(_) = self.parse_flac_marker(&buffer) {
            println!("not include flac header");
            return Ok(());
        }
        buffer = buffer_reader.read_flac_header()?;
        let mut block_header = self.parse_block_header(&buffer)?;
        println!("block_header: {:?}", block_header);
        if let BlockType::STREAMINFO = block_header.block_type {
            if block_header.length == 34 {
                buffer = buffer_reader.read_block_data_buffer(block_header.length)?;
                let strinfo = self.parse_stream_info_block(&buffer)?;
                println!("stream info: {:?}", strinfo);
            } else {
                println!("broken stream info block");
            }
        } else {
            println!("the first block is not stream info");
            return Ok(());
        }
        while !block_header.is_last {
            buffer = buffer_reader.read_flac_header()?;
            block_header = self.parse_block_header(&buffer)?;
            match block_header.block_type {
                BlockType::STREAMINFO => todo!(),
                BlockType::PADDING => {
                    println!("here is padding, is last = {}", block_header.is_last);
                },
                BlockType::APPLICATION => {
                    buffer = buffer_reader.read_block_data_buffer(block_header.length)?;
                    self.parse_block_application(&buffer).unwrap();
                }
                BlockType::SEEKTABLE => {
                    buffer = buffer_reader.read_block_data_buffer(block_header.length)?;
                    self.parse_block_seektable(&buffer).unwrap()
                }
                BlockType::VORBISCOMMENT => {
                    buffer = buffer_reader.read_block_data_buffer(block_header.length)?;
                    self.parse_vorbis_comment(&buffer).unwrap();
                    println!("vorbis = {:?}", self.vorbis);
                    // return Ok(());
                }
                BlockType::CUESHEET => todo!(),
                BlockType::PICTURE => {
                    buffer = buffer_reader.read_block_data_buffer(block_header.length)?;
                    self.parse_block_picture(&buffer).unwrap();
                    // return Ok(());
                }
                BlockType::INVALID => todo!(),
            }
        }
        Ok(())
    }
    fn parse_flac_marker(&mut self, buffer: &Vec<u8>) -> Result<(), FlacError> {
        if buffer[..] == [0x66, 0x4C, 0x61, 0x43] {
            return Ok(());
        }
        let marker = String::from_utf8(buffer.clone()).unwrap();
        println!("marker = {}", marker);
        Err(FlacError::WrongHeader)
    }
    fn parse_block_header(&mut self, buffer: &Vec<u8>) -> io::Result<BlockHeader> {
        let is_last = (buffer[0] & 0x80) == 0x80;
        let block_type: BlockType = match buffer[0] & 0x7F {
            0 => BlockType::STREAMINFO,
            1 => BlockType::PADDING,
            2 => BlockType::APPLICATION,
            3 => BlockType::SEEKTABLE,
            4 => BlockType::VORBISCOMMENT,
            5 => BlockType::CUESHEET,
            6 => BlockType::PICTURE,
            _ => BlockType::INVALID,
        };
        let length = buffer[1] as u32 * 0x10000 + buffer[2] as u32 * 0x100 + buffer[3] as u32;
        Ok(BlockHeader::new(is_last, block_type, length as u32))
    }

    fn parse_stream_info_block(&mut self, buffer: &Vec<u8>) -> io::Result<StreamInfo> {
        let min_block_size: u16 = buffer[0] as u16 * 0x100 + buffer[1] as u16; // 2 Bytes
        let max_block_size: u16 = buffer[2] as u16 * 0x100 + buffer[3] as u16; // 2 Bytes
        let min_frame_size: u32 =
            buffer[4] as u32 * 0x10000 + buffer[5] as u32 * 0x100 + buffer[6] as u32; // 3 Bytes
        let max_frame_size: u32 =
            buffer[7] as u32 * 0x10000 + buffer[8] as u32 * 0x100 + buffer[9] as u32; // 3Bytes

        let sample_rate: u32 = buffer[10] as u32 * 0x1000
            + buffer[11] as u32 * 0x10
            + ((buffer[12] & 0xF0) >> 4) as u32; // 2 Bytes + 4 bits
        let channels: u8 = ((buffer[12] & 0x0F) >> 1) + 1; // 3 bits
        let bits_per_sample: u8 = ((buffer[12] & 0x01) * 0x10) + ((buffer[13] & 0xF0) >> 4) + 1; // 1 bit + 4 bits

        let total_samples: u64 = (buffer[13] & 0x0F) as u64 * 0x100000000
            + buffer[14] as u64 * 0x1000000
            + buffer[15] as u64 * 0x10000
            + buffer[16] as u64 * 0x100
            + buffer[17] as u64; // 4 bits + 4 Bytes
        let md5: u128 = buffer[18] as u128 * 0x1000000000000000000000000000000
            + buffer[19] as u128 * 0x10000000000000000000000000000
            + buffer[20] as u128 * 0x100000000000000000000000000
            + buffer[21] as u128 * 0x1000000000000000000000000
            + buffer[22] as u128 * 0x10000000000000000000000
            + buffer[23] as u128 * 0x100000000000000000000
            + buffer[24] as u128 * 0x1000000000000000000
            + buffer[25] as u128 * 0x10000000000000000
            + buffer[26] as u128 * 0x100000000000000
            + buffer[27] as u128 * 0x1000000000000
            + buffer[28] as u128 * 0x10000000000
            + buffer[29] as u128 * 0x100000000
            + buffer[30] as u128 * 0x1000000
            + buffer[31] as u128 * 0x10000
            + buffer[32] as u128 * 0x100
            + buffer[33] as u128 * 0x1; // the last 16 Bytes(=128 bits)
        Ok(StreamInfo::new(
            min_block_size,
            max_block_size,
            min_frame_size,
            max_frame_size,
            sample_rate,
            channels,
            bits_per_sample,
            total_samples,
            md5,
        ))
    }
    fn parse_vorbis_comment(&mut self, buf: &Vec<u8>) -> io::Result<()> {
        let buffer: Vec<u8> = buf.to_owned();
        let mut start = 0;
        let mut end = 3;
        let encoder_length = self.parse_length(&buffer[start..=end]);

        start = end + 1;
        end = start - 1 + encoder_length as usize;
        let encoder: String = String::from_utf8(buffer[start..=end].to_vec()).unwrap();

        start = end + 1;
        end = start - 1 + 4;
        let tags_number = self.parse_length(&buffer[start..=end]);

        let mut tag_index = 0;
        while tag_index < tags_number {
            tag_index += 1;
            start = end + 1;
            end = start - 1 + 4;
            let tag_length = self.parse_length(&buffer[start..=end]);
            start = end + 1;
            end = start - 1 + tag_length as usize;
            let tag_content_raw = String::from_utf8(buffer[start..=end].to_vec()).unwrap();
            let kv: Vec<&str> = tag_content_raw.split("=").collect();
            let tag_key: String = kv[0].to_owned();
            let tag_value = kv[1].to_owned();
            if let Some(index) = self.vorbis_hash.get(&tag_key) {
                self.vorbis[*index].push(tag_value);
            } else {
                let len = self.vorbis.len();
                self.vorbis_hash.insert(tag_key, len);
                self.vorbis.push(Vec::default());
                self.vorbis[len].push(tag_value);
            }
        }
        Ok(())
    }
    fn parse_block_picture(&mut self, buf: &Vec<u8>) -> io::Result<()> {
        let buffer: Vec<u8> = buf.to_owned();
        let mut start = 0;
        let mut end = 3;
        let pic_type: PicType = PicType::from(self.parse_length_2(&buffer[start..=end]) as u8);
        println!("pic_type: {:?}", pic_type);
        start = end + 1;
        end = start - 1 + 4;
        let mime_length = self.parse_length_2(&buffer[start..=end]);
        println!("mime_length: {}", mime_length);
        start = end + 1;
        end = start - 1 + mime_length as usize;
        let mime_string: String = String::from_utf8(buffer[start..=end].to_vec()).unwrap();

        start = end + 1;
        end = start - 1 + 4;
        let desc_length = self.parse_length_2(&buffer[start..=end]);

        start = end + 1;
        end = start - 1 + desc_length as usize;
        let desc_string: String = String::from_utf8(buffer[start..=end].to_vec()).unwrap();

        start = end + 1;
        end = start - 1 + 4;
        let pic_width = self.parse_length_2(&buffer[start..=end]);

        start = end + 1;
        end = start - 1 + 4;
        let pic_height = self.parse_length_2(&buffer[start..=end]);

        start = end + 1;
        end = start - 1 + 4;
        let bit_depth = self.parse_length_2(&buffer[start..=end]);

        start = end + 1;
        end = start - 1 + 4;
        let index_color_number = self.parse_length_2(&buffer[start..=end]);

        start = end + 1;
        end = start - 1 + 4;
        let pic_length = self.parse_length_2(&buffer[start..=end]);

        start = end + 1;
        let pic_data: Vec<u8> = buffer[start..].to_vec();

        println!("pic_width: {}", pic_width);
        println!("pic_height: {}", pic_height);
        println!("pic_length = {}", pic_length);
        println!("pic_data_last = {}", pic_data.last().unwrap());

        Ok(())
    }
    fn parse_block_application(&mut self, buf: &Vec<u8>) -> io::Result<()> {
        let id = self.parse_length_2(&buf[0..=3]);
        let application_data: Vec<u8> = buf[4..].to_owned();
        Ok(())
    }
    fn parse_block_seektable(&mut self, buf: &Vec<u8>) -> io::Result<()> {
        let buffer = buf.to_owned();
        let mut start = 0;
        let mut end = 7; // for the first loop
        let length = buf.len();
        let mut index = 0;
        let mut seek_table: SeekTable = SeekTable::default();
        while index < length {
            let mut seek_point = SeekPoint::default();
            index += 1;

            let snfs = self.parse_8_bytes(&buffer[start..=end]);
            seek_point.sample_number_of_first_sample = snfs;
            if snfs == 0xFFFFFFFFFFFFFFFF {
                seek_table.seekpoints.push(seek_point);
                start = end + 1;
                end = start - 1 + 8; // for next loop
                continue;
            }
            start = end + 1;
            end = start -1 + 8;
            let offset = self.parse_8_bytes(&buffer[start..=end]);
            seek_point.offset = offset;
            
            start = end + 1;
            end = start - 1 + 2;
            let na = buffer[start] as u16 * 0x100 + buffer[end] as u16;
            seek_point.number_of_samples = na;

            seek_table.seekpoints.push(seek_point);

            start = end + 1;
            start = end - 1 + 8; // for next loop
        }
        Ok(())
    }
    fn parse_length(&mut self, buffer: &[u8]) -> u32 {
        buffer[0] as u32
            + buffer[1] as u32 * 0x100
            + buffer[2] as u32 * 0x10000
            + buffer[3] as u32 * 0x1000000
    }
    fn parse_length_2(&mut self, buffer: &[u8]) -> u32 {
        buffer[3] as u32
            + buffer[2] as u32 * 0x100
            + buffer[1] as u32 * 0x10000
            + buffer[0] as u32 * 0x1000000
    }
    fn parse_8_bytes(&mut self, buffer: &[u8]) -> u64 {
        buffer[7] as u64
            + buffer[6] as u64 * 0x100
            + buffer[5] as u64 * 0x10000
            + buffer[4] as u64 * 0x1000000
            + buffer[3] as u64 * 0x100000000
            + buffer[2] as u64 * 0x10000000000
            + buffer[1] as u64 * 0x1000000000000
            + buffer[0] as u64 * 0x100000000000000
    }
}
