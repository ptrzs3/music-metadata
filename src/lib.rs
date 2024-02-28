// 如果frame有description不定长字段，如果frame中为空，则设置为String::from("null")
mod flac;
mod id3;
mod ogg;
mod util;

use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;

use flac::blocks::{
    block_application::BlockApplication,
    block_cue_sheet::BlockCueSheet,
    block_header::{BlockHeader, BlockType},
    block_picture::BlockPicture,
    block_seektable::BlockSeekTable,
    block_stream_info::BlockStreamInfo,
    block_vorbis_comment::BlockVorbisComment,
};
use flac::core::parse_block_cue_sheet;
use flac::flac_buffer_reader::FlacBufferReader;
use id3::{
    core::{
        parse_extended_header, parse_footer_buffer, parse_frame_header, parse_frame_payload,
        parse_protocol_header,
    },
    frames::APIC::PicType,
};
use id3::{
    error::ID3Error, extended_header::ExtendedHeader, footer::Footer, frames::common::Tape,
    id3_buffer_reader::ID3BufferReader, id3v1_tag::ID3v1, protocol_header::ProtocolHeader,
};

use ogg::{
    ogg_buffer_reader::OggBufferReader,
    ogg_vorbis_comment::{CommentBody, CommentHeader, HeaderType},
    page::PageHeader,
};
use util::{parse_4_bytes_with_little_endian, Buffer};

use flac::core::{
    parse_block_application, parse_block_header, parse_block_picture, parse_block_seektable,
    parse_flac_marker, parse_stream_info_block, parse_vorbis_comment,
};

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
    /// 传入一个列表，启动多个线程进行解析：怎么返回值？线程切换成本？
    /// 用户启动多个线程，每个线程一个ID3Parser
    /// 异步？对已经确定的Buffer
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

    /// Start parsing id3v1.
    ///
    /// It is not recommended to call this method,
    ///
    /// thinking that the ID3 protocol contains very little information,
    ///
    /// unless a very old song.
    pub fn parse_id3v1(&mut self) -> io::Result<()> {
        let position = self.file_size - 128;
        let mut buffer_reader = ID3BufferReader::new(&self.fp)?;
        buffer_reader.seek_to(position)?;
        let buffer = buffer_reader.read_id3v1_buffer()?;
        let mut start: usize = 0;
        let header: Vec<u8> = (buffer[start..start + 3]).to_vec();
        // update_start_end()
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

    /// Start parsing id3v2.
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
                    // 这里可以优化为异步
                    // 去解析而不等待返回值，接着获取下一个FrameHeader继续解析
                    buffer = buffer_reader.read_frame_payload_buffer(v.size)?;
                    // 优化为异步
                    match parse_frame_payload(&buffer, &v) {
                        Ok(v) => {
                            self.push(v)?;
                        }
                        Err(e) => println!("{:?}", e),
                    }
                    start += 10 + v.size;
                }
                Err(e) => match e {
                    ID3Error::IsPadding => {
                        self.padding_size = self.pheader.size - start;
                        if self.pheader.flags.Footer {
                            // 将reader的指针定位到footer第一个字节
                            buffer_reader.seek_to(10 + self.pheader.size as u64)?;
                            buffer = buffer_reader.read_footer_buffer()?;
                            self.footer = parse_footer_buffer(&buffer).unwrap();
                        }
                        return Ok(());
                    }
                    ID3Error::Unimplement(id, skip) => {
                        let buf = buffer_reader.skip(skip)?;
                        start += 10 + skip;
                        println!(
                            "unimplement: {{
identifier: {},
raw: {:?}",
                            id, buf
                        );
                    }
                    ID3Error::UnknownError(s) => {
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
                let pic_type = PicType::from(d.raw().pop().unwrap()).to_string();
                let mut fname: OsString = OsString::from(&t);
                fname.push("_mp3_");
                fname.push(pic_type);
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
#[derive(Debug)]
pub struct FlacParser<T>
where
    T: AsRef<Path>,
{
    fp: T,
    pub stream_info: BlockStreamInfo,
    pub application: BlockApplication,
    pub seek_table: BlockSeekTable,
    vorbis_comment: BlockVorbisComment,
    pub picture: Vec<BlockPicture>,
    pub cue_sheet: BlockCueSheet,
    pub padding_length: u32,
}

#[allow(dead_code)]
#[allow(unused_assignments)]
#[allow(unused_variables)]
impl<T> FlacParser<T>
where
    T: AsRef<Path>,
{
    /// Create a new FlacParser
    pub fn new(fp: T) -> io::Result<Self> {
        Ok(FlacParser {
            fp,
            stream_info: BlockStreamInfo::default(),
            application: BlockApplication::default(),
            seek_table: BlockSeekTable::default(),
            vorbis_comment: BlockVorbisComment::default(),
            picture: Vec::default(),
            cue_sheet: BlockCueSheet::default(),
            padding_length: u32::default(),
        })
    }

    /// Start parsing flac.
    pub fn parse(&mut self) -> io::Result<()> {
        let mut buffer_reader = FlacBufferReader::new(&self.fp)?;
        let mut buffer: Buffer;
        buffer = buffer_reader.read_block_header()?;
        if parse_flac_marker(buffer).is_err() {
            println!("not include flac header");
            return Ok(());
        }
        let mut block_header: BlockHeader = BlockHeader::default();
        while !block_header.is_last {
            buffer = buffer_reader.read_block_header()?;
            block_header = parse_block_header(buffer)?;
            buffer = buffer_reader.read_block_data_buffer(block_header.length)?;
            match block_header.block_type {
                BlockType::STREAMINFO => {
                    self.stream_info = parse_stream_info_block(buffer)?;
                }
                BlockType::PADDING => {
                    self.padding_length = block_header.length;
                    println!("here is padding, is last = {}", block_header.is_last);
                }
                BlockType::APPLICATION => {
                    self.application = parse_block_application(buffer)?;
                }
                BlockType::SEEKTABLE => {
                    self.seek_table = parse_block_seektable(buffer)?;
                }
                BlockType::VORBISCOMMENT => {
                    self.vorbis_comment = parse_vorbis_comment(buffer)?;
                }
                BlockType::CUESHEET => {
                    self.cue_sheet = parse_block_cue_sheet(buffer)?;
                }
                BlockType::PICTURE => {
                    self.picture.push(parse_block_picture(buffer)?);
                }
                BlockType::INVALID => todo!(),
            }
        }
        Ok(())
    }

    /// Get vorbis comment according to query.
    ///
    /// Return a Vec<String> wrapped in an Option.
    pub fn get(&mut self, query: &str) -> Option<Vec<String>> {
        let upper_query = query.to_uppercase();
        if let Some(index) = self.vorbis_comment.hm.get(&upper_query) {
            return Some(self.vorbis_comment.comment[*index].clone());
        }
        None
    }

    /// Given that Vorbis allows for customized key values,
    ///
    /// there may be key values other than those in common use,
    ///
    /// so this method is provided to print all key-value pairs.
    pub fn get_all(&mut self) -> io::Result<(Vec<String>, Vec<Vec<String>>)> {
        let mut key_vec: Vec<String> = Vec::default();
        let mut value_vec: Vec<Vec<String>> = Vec::default();
        for (key, index) in &self.vorbis_comment.hm {
            key_vec.push(key.to_string());
            value_vec.push(self.vorbis_comment.comment[*index].clone());
        }
        Ok((key_vec, value_vec))
    }

    /// Write image(s) to disk.
    pub fn write_image(&mut self) -> io::Result<()> {
        let mut t = self.fp.as_ref().to_owned();
        t.set_extension("");
        let mut index = 0;
        while index < self.picture.len() {
            let mut raw_data = self.picture[index].data.clone();
            let pic_type = PicType::from(raw_data.pop().unwrap()).to_string();
            let mut fname: OsString = OsString::from(&t);
            fname.push("_flac_");
            fname.push(pic_type);
            if index > 0 {
                fname.push("_");
                fname.push(index.to_string());
            }
            fname.push(".jpg");
            fs::write(fname, raw_data)?;
            index += 1;
        }
        Ok(())
    }

    /// As the method says.
    ///
    /// In addition, its own data will be cleared.
    pub fn change_target(&mut self, new_fp: T) {
        self.fp = new_fp;
        self.application = BlockApplication::default();
        self.stream_info = BlockStreamInfo::default();
        self.seek_table = BlockSeekTable::default();
        self.vorbis_comment = BlockVorbisComment::default();
        self.picture.clear();
        self.cue_sheet = BlockCueSheet::default();
        self.padding_length = u32::default();
    }
}

pub struct OggParser<T>
where
    T: AsRef<Path>,
{
    fp: T,
    pub audio_channels: u8,
    pub audio_sample_rate: u32,
    pub vorbis_comment: CommentBody,
    // pub vorbis_comment: Box<dyn FlagTrait>,
    comment_buffer: Vec<u8>,
}
impl<T> OggParser<T>
where
    T: AsRef<Path>,
{
    pub fn new(fp: T) -> Self {
        OggParser {
            fp,
            audio_channels: u8::default(),
            audio_sample_rate: u32::default(),
            // vorbis_comment: Box::new(OggVorbisComment::default()),
            vorbis_comment: CommentBody::default(),
            comment_buffer: Vec::default(),
        }
    }
    pub fn parse(&mut self) -> io::Result<()> {
        let mut buffer_reader = OggBufferReader::new(&self.fp)?;
        let mut comment_header = CommentHeader::default();
        // comment缓存
        // let mut comment_buffer: Vec<u8> = Vec::default();
        while !comment_header.end {
            let mut page_header = PageHeader::default();
            page_header.capture_pattern = String::from_utf8(buffer_reader.read_buffer(4)?).unwrap();
            page_header.structure_version = buffer_reader.read_one()?;
            let header_type_flag = buffer_reader.read_one()?;
            page_header.new_packet = (header_type_flag & 0x01) == 0;
            page_header.bos = (header_type_flag & 0x02) == 2;
            page_header.eos = (header_type_flag & 0x04) == 4;
            page_header.granule_position = buffer_reader.read_buffer(8)?;
            page_header.serial_number = buffer_reader.read_buffer(4)?;
            page_header.page_sequence_number = buffer_reader.read_buffer(4)?;
            page_header.crc_checksum = buffer_reader.read_buffer(4)?;
            page_header.number_page_segments = buffer_reader.read_one()?;
            page_header.segment_table =
                buffer_reader.read_buffer(page_header.number_page_segments as u32)?;
            let temp = buffer_reader.read_one().unwrap();
            match temp {
                0x01 | 0x05 => {
                    let skip_len: u64 = page_header
                        .segment_table
                        .clone()
                        .into_iter()
                        .map(|x| x as u64)
                        .sum();
                    buffer_reader.skip(skip_len - 1)?;
                    continue;
                }
                _ => {}
            }
            let mut packets: Vec<u32> = Vec::new();
            packets.push(0);
            // 此页有几个包，包的长度是多少
            for (i, ele) in page_header.segment_table.into_iter().enumerate() {
                // 非0xFF
                if ele != 0xFF {
                    *packets.last_mut().unwrap() += ele as u32;
                    if i != page_header.number_page_segments as usize - 1 {
                        packets.push(0);
                    }
                } else {
                    *packets.last_mut().unwrap() += 0xFF;
                }
            }
            // 第一个包长度-1
            packets[0] -= 0x01;
            // 是一个新包，并且有区段，并且第一个字节是0x03
            if page_header.new_packet && page_header.number_page_segments > 0 && temp == 0x03 {
                // 将所有数据读入
                for len in packets {
                    self.comment_buffer
                        .append(buffer_reader.read_buffer(len)?.as_mut());
                }
                // 进入下一页
            } else {
                // 是继承包
                self.comment_buffer.push(temp);
                for len in packets {
                    self.comment_buffer
                        .append(buffer_reader.read_buffer(len)?.as_mut());
                    while *self.comment_buffer.last().unwrap() == 0x00 {
                        self.comment_buffer.pop();
                    }
                    if *self.comment_buffer.last().unwrap() == 0x01 {
                        self.comment_buffer.pop();
                        comment_header.end = true;
                        break;
                    }
                }
            }
        }
        comment_header.header_type = HeaderType::CommentHeader;
        comment_header.packet_pattern = String::from_utf8(self.vec_reader(0, 6).to_vec()).unwrap();
        let company_info_length: usize =
            parse_4_bytes_with_little_endian(self.vec_reader(6, 4)) as usize;
        comment_header.company_info =
            String::from_utf8(self.vec_reader(10, company_info_length).to_vec()).unwrap();
        let mut start: usize = 10 + company_info_length + 4;
        while self.comment_buffer.len() - start > 1 {
            let comment_i_length: usize =
                parse_4_bytes_with_little_endian(self.vec_reader(start, 4)) as usize;
            start += 4;
            let comment_i_content =
                String::from_utf8(self.vec_reader(start, comment_i_length).to_vec()).unwrap();
            start += comment_i_length;
            // store(&mut self.vorbis_comment, comment_i_content);
            // self.vorbis_comment.store(comment_i_content);
            let kv: Vec<&str> = comment_i_content.split('=').collect();
            let comment_key = kv[0];
            let comment_value: String = kv[1].to_owned();
            if let Some(index) = self.vorbis_comment.hm.get(comment_key) {
                self.vorbis_comment.comment[*index].push(comment_value);
            } else {
                let comment_index = self.vorbis_comment.comment.len();
                self.vorbis_comment
                    .hm
                    .insert(comment_key.to_uppercase(), comment_index);
                self.vorbis_comment.comment.push(Vec::default());
                self.vorbis_comment.comment[comment_index].push(comment_value);
            }
        }
        Ok(())
    }
    // return buffer
    fn vec_reader(&self, start: usize, length: usize) -> &[u8] {
        &self.comment_buffer[start..start + length]
    }
    pub fn get(&mut self, query: &str) -> Option<Vec<String>> {
        let upper_query = query.to_uppercase();
        if let Some(index) = self.vorbis_comment.hm.get(&upper_query) {
            return Some(self.vorbis_comment.comment[*index].clone());
        }
        None
    }
    pub fn get_all(&mut self) -> io::Result<(Vec<String>, Vec<Vec<String>>)> {
        let mut key_vec: Vec<String> = Vec::default();
        let mut value_vec: Vec<Vec<String>> = Vec::default();
        for (key, index) in &self.vorbis_comment.hm {
            key_vec.push(key.to_string());
            value_vec.push(self.vorbis_comment.comment[*index].clone());
        }
        Ok((key_vec, value_vec))
    }
}
