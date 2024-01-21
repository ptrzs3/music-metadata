// 如果frame有description不定长字段，如果frame中为空，则设置为String::from("null")

mod error;
mod extended_header;
mod frames;
mod id3v1_tag;
mod protocol_header;
mod reader;
mod util;
mod version;

use error::header_error::HeaderError;
use extended_header::ExtendedHeader;
use frames::common::Tape;
use frames::header::FrameHeader;
use frames::identifiers::{
    IDFactory, RarelyUsedFrameIdentifier, TextInformationFrameIdentifier, URLLinkFrameIdentifier,
};
use id3v1_tag::ID3v1;
use protocol_header::{Flag, ProtocolHeader};
use reader::{Buffer, BufferReader};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::{fs, io};
use version::Version;

pub struct Parser<T>
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
    padding_size: u32,
    /// ID3v1 tag
    pub id3v1: ID3v1,
    /// file size, for locating ID3v1
    file_size: u64,
}

impl<T> Parser<T>
where
    T: AsRef<Path>,
{
    /// Create a new parser.
    pub fn new(fp: T) -> io::Result<Self> {
        let file_size = File::open(&fp)?.metadata()?.len();
        Ok(Parser {
            fp,
            hm: HashMap::default(),
            frames: Vec::default(),
            pheader: ProtocolHeader::default(),
            eheader: ExtendedHeader::default(),
            padding_size: u32::default(),
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
    /// APIC should call this method, as should SYLT,
    /// SYLT may call the `get` method in the future
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
        let mut buffer_reader = BufferReader::new(&self.fp)?;
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
        let mut buffer_reader = BufferReader::new(&self.fp)?;

        let mut buffer: Buffer;

        buffer = buffer_reader.read_protocol_header_buffer()?;
        let rst = parse_protocol_header(&buffer);
        if let Err(_) = rst {
            println!("not include ID3v2.3 or ID3v2.4");
            return Ok(());
        }
        self.pheader = rst.unwrap();
        let mut start: u32 = 0;
        if self.pheader.flags.ExtendedHeader {
            buffer = buffer_reader.read_extended_header()?;
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
                        println!("### Endding ###");
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
                let fname = t.as_mut_os_string();
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
fn parse_protocol_header(header: &Buffer) -> Result<ProtocolHeader, HeaderError> {
    let header = ProtocolHeader {
        identifier: String::from_utf8_lossy(&header[..3]).into_owned(),
        major_version: {
            if header[0..=3] == [0x49, 0x44, 0x33, 0x03] {
                Version::V3
            } else if header[0..=3] == [0x49, 0x44, 0x33, 0x04] {
                Version::V4
            } else {
                return Err(HeaderError::Unimplement("Wrong Header".to_string(), 1));
            }
        },
        revision: header[4],
        size: util::get_size(header[6..].to_vec(), &Version::V4),
        flags: Flag::new(header[5]),
    };
    Ok(header)
}

fn parse_extended_header(header: &Buffer, version: &Version) -> ExtendedHeader {
    ExtendedHeader::new(
        version.clone(),
        util::get_size(header[0..=3].to_vec(), version) as u8,
        header[4..].into(),
    )
}
fn parse_frame_header(header: &Buffer, version: &Version) -> Result<FrameHeader, HeaderError> {
    let frame_header = FrameHeader {
        identifier: IDFactory::from(header[0..=3].to_vec()),
        size: util::get_size(header[4..8].to_vec(), version),
        flags: util::map_to_binary(&header[8..]),
        version: version.clone(),
    };
    if let IDFactory::R(RarelyUsedFrameIdentifier::UNIMPLEMENT(id)) = frame_header.identifier {
        return Err(HeaderError::Unimplement(id, frame_header.size));
    }
    if let IDFactory::PADDING = frame_header.identifier {
        return Err(HeaderError::IsPadding);
    }
    Ok(frame_header)
}

fn parse_frame_payload(
    payload: &Buffer,
    header: &FrameHeader,
) -> Result<Box<dyn Tape>, HeaderError> {
    match &header.identifier {
        IDFactory::T(id) => {
            if let TextInformationFrameIdentifier::TXXX = id {
                let txxx = worker::parse_TXXX(payload.clone())?;
                return Ok(Box::new(txxx));
            } else {
                let text_infomation_frame = worker::parse_text_infomation_frame(
                    header.identifier.to_string(),
                    payload.clone(),
                )?;
                return Ok(Box::new(text_infomation_frame));
            }
        }
        IDFactory::W(id) => {
            if let URLLinkFrameIdentifier::WXXX = id {
                let wxxx = worker::parse_WXXX(payload.clone())?;
                return Ok(Box::new(wxxx));
            } else {
                let url_link_frame =
                    worker::parse_url_link_frame(header.identifier.to_string(), payload.clone())?;
                return Ok(Box::new(url_link_frame));
            }
        }
        IDFactory::APIC => {
            let apic = worker::parse_APIC(payload.clone())?;
            return Ok(Box::new(apic));
        }
        IDFactory::COMM => {
            let comm = worker::parse_COMM(payload.clone())?;
            return Ok(Box::new(comm));
        }
        IDFactory::USLT => {
            let uslt = worker::parse_USLT(payload.clone())?;
            return Ok(Box::new(uslt));
        }
        IDFactory::SYLT => {
            let sylt = worker::parse_SYLT(payload.clone())?;
            return Ok(Box::new(sylt));
        }
        IDFactory::R(_) => {
            let rarely_used =
                worker::parse_RarelyUsed(header.identifier.to_string(), payload.clone())?;
            return Ok(Box::new(rarely_used));
        }
        _ => {
            panic!("Unexpected Error")
        }
    }
}

mod worker {
    use crate::{
        error::header_error::HeaderError,
        frames::{
            common::Encoding,
            rarely_used::RarelyUsed,
            text_infomation_frame::TextInfomationFrame,
            url_link_frame::URLLinkFrame,
            APIC::{PicType, APIC},
            COMM::COMM,
            SYLT::SYLT,
            TXXX::TXXX,
            USLT::USLT,
            WXXX::WXXX,
        },
        reader::Buffer,
    };

    use super::common;

    #[allow(non_snake_case)]
    pub fn parse_text_infomation_frame(
        identifier: String,
        payload: Buffer,
    ) -> Result<TextInfomationFrame, HeaderError> {
        let mut encoding = common::get_encoding(payload[0])?;
        let mut cursor: usize = 1;
        if let Encoding::UTF16_WITH_BOM = encoding {
            encoding = common::refine_encoding(&payload[1..=2]);
            cursor += 2;
        }
        let data = common::get_text(&encoding, &payload[cursor..])?;
        Ok(TextInfomationFrame::new(identifier, data))
    }

    pub fn parse_url_link_frame(
        identifier: String,
        payload: Buffer,
    ) -> Result<URLLinkFrame, HeaderError> {
        let data = common::get_text(&Encoding::UTF8, &payload[..])?;
        Ok(URLLinkFrame::new(identifier, data))
    }

    #[allow(non_snake_case)]
    pub fn parse_TXXX(payload: Buffer) -> Result<TXXX, HeaderError> {
        let mut encoding = common::get_encoding(payload[0])?;
        let mut cursor = 1;
        if let Encoding::UTF16_WITH_BOM = encoding {
            encoding = common::refine_encoding(&payload[cursor..=cursor + 1]);
            cursor += 2;
        }
        let (description, skip) =
            common::get_text_according_to_encoding(&payload[cursor..], &encoding)?;
        cursor += skip;
        let data = common::get_text(&Encoding::UTF8, &payload[cursor..])?;
        Ok(TXXX::new(encoding, description, data))
    }

    #[allow(non_snake_case)]
    pub fn parse_WXXX(payload: Buffer) -> Result<WXXX, HeaderError> {
        let mut encoding = common::get_encoding(payload[0])?;
        let mut cursor = 1;
        if let Encoding::UTF16_WITH_BOM = encoding {
            encoding = common::refine_encoding(&payload[cursor..=cursor + 1]);
            cursor += 2;
        }
        let (description, skip) =
            common::get_text_according_to_encoding(&payload[cursor..], &encoding)?;
        cursor += skip;
        let data = common::get_text(&Encoding::UTF8, &payload[cursor..])?;
        Ok(WXXX::new(encoding, description, data))
    }

    #[allow(non_snake_case)]
    pub fn parse_USLT(payload: Buffer) -> Result<USLT, HeaderError> {
        let frame_encoding = common::get_encoding(payload[0])?;
        let mut data_encoding = common::get_encoding(payload[0])?;
        let language: String = String::from_utf8(payload[1..=3].into()).expect("");
        let mut cursor: usize = 4;
        if let Encoding::UTF16_WITH_BOM = frame_encoding {
            data_encoding = common::refine_encoding(&payload[cursor..=cursor + 1]);
            cursor += 2;
        }
        let (description, skip): (String, usize) =
            common::get_text_according_to_encoding(&payload[cursor..], &data_encoding)?;
        cursor += skip;
        if let Encoding::UTF16_WITH_BOM = frame_encoding {
            data_encoding = common::refine_encoding(&payload[cursor..=cursor + 1]);
            cursor += 2;
        }
        let data = common::get_text(&data_encoding, &payload[cursor..])?;
        Ok(USLT::new(data_encoding, language, description, data.into()))
    }

    #[allow(non_snake_case)]
    pub fn parse_SYLT(payload: Buffer) -> Result<SYLT, HeaderError> {
        let frame_encoding = common::get_encoding(payload[0])?;
        let language: String = String::from_utf8(payload[1..=3].into()).expect("");
        let timestamp_format: u8 = payload[4];
        let ctype: u8 = payload[5];
        let mut cursor: usize = 6;
        let (description, skip) =
            common::get_text_according_to_encoding(&payload[cursor..], &frame_encoding)?;
        cursor += skip;
        let data: Vec<u8> = payload[cursor..].into();
        Ok(SYLT::new(
            frame_encoding,
            language,
            timestamp_format,
            ctype,
            description,
            data,
        ))
    }

    #[allow(non_snake_case)]
    pub fn parse_COMM(payload: Buffer) -> Result<COMM, HeaderError> {
        let frame_encoding = common::get_encoding(payload[0])?;
        let mut data_encoding = common::get_encoding(payload[0])?;
        let language: String = String::from_utf8(payload[1..=3].into()).expect("");
        let mut cursor: usize = 4;
        if let Encoding::UTF16_WITH_BOM = frame_encoding {
            data_encoding = common::refine_encoding(&payload[cursor..=cursor + 1]);
            cursor += 2;
        }
        let (description, skip): (String, usize) =
            common::get_text_according_to_encoding(&payload[cursor..], &data_encoding)?;
        cursor += skip;
        if let Encoding::UTF16_WITH_BOM = frame_encoding {
            data_encoding = common::refine_encoding(&payload[cursor..=cursor + 1]);
            cursor += 2;
        }
        let data = common::get_text(&data_encoding, &payload[cursor..])?;
        Ok(COMM::new(data_encoding, language, description, data))
    }

    #[allow(non_snake_case)]
    pub fn parse_APIC(payload: Buffer) -> Result<APIC, HeaderError> {
        let mut encoding = common::get_encoding(payload[0])?;
        let mut cursor: usize = 1;
        let (MIME_type, skip): (String, usize) =
            common::get_text_according_to_encoding(&payload[cursor..], &Encoding::UTF8)?;
        cursor += skip;
        let picture_type: PicType = PicType::from(payload[cursor]);
        cursor += 1;
        if let Encoding::UTF16_WITH_BOM = encoding {
            encoding = common::refine_encoding(&payload[cursor..=cursor + 1]);
            cursor += 2;
        }
        let (description, skip): (String, usize) =
            common::get_text_according_to_encoding(&payload[cursor..], &encoding)?;
        cursor += skip;
        let data: Vec<u8> = payload[cursor..].into();
        Ok(APIC::new(
            encoding,
            MIME_type,
            picture_type,
            description,
            data,
        ))
    }

    #[allow(non_snake_case)]
    pub fn parse_RarelyUsed(
        identifier: String,
        payload: Buffer,
    ) -> Result<RarelyUsed, HeaderError> {
        Ok(RarelyUsed::new(identifier, payload))
    }
}

mod common {

    use std::collections::VecDeque;

    use crate::{error::header_error::HeaderError, frames::common::Encoding, util};

    pub fn get_text(encoding: &Encoding, payload: &[u8]) -> Result<String, HeaderError> {
        let text = match encoding {
            Encoding::ISO_8859_1 => util::latin1_to_string(payload),
            Encoding::UTF16_BE => {
                String::from_utf16(&util::into_big_endian_u16(payload, false)?).expect("")
            }
            Encoding::UTF16_LE => {
                String::from_utf16(&util::into_big_endian_u16(payload, true)?).expect("")
            }
            Encoding::UTF8 => {
                // 掐头去尾，防止出现0x00
                let mut vq: VecDeque<u8> = VecDeque::from(Into::<Vec<u8>>::into(payload));
                while let Some(0x00) = vq.front() {
                    vq.pop_front();
                }
                while let Some(0x00) = vq.back() {
                    vq.pop_back();
                }
                String::from_utf8(vq.into()).expect("")
            }
            _ => "".to_string(),
        };
        Ok(text)
    }

    pub fn get_encoding(payload: u8) -> Result<Encoding, HeaderError> {
        let encoding = match payload {
            0x00 => Encoding::ISO_8859_1,
            0x01 => Encoding::UTF16_WITH_BOM,
            0x02 => Encoding::UTF16_BE,
            0x03 => Encoding::UTF8,
            _ => {
                return Err(HeaderError::UnknownError(
                    "Out-of-bounds indexing".to_string(),
                ))
            }
        };
        Ok(encoding)
    }

    pub fn get_text_according_to_encoding(
        payload: &[u8],
        encoding: &Encoding,
    ) -> Result<(String, usize), HeaderError> {
        let mut cursor: usize = 0;
        let mut text_vec: Vec<u8> = Vec::new();
        let mut text: String;
        match encoding {
            Encoding::ISO_8859_1 => {
                while cursor < payload.len() && payload[cursor] != 0 {
                    text_vec.push(payload[cursor]);
                    cursor += 1;
                }
                text = util::latin1_to_string(&text_vec);
                if text.len() == 0 {
                    text = "null".to_string();
                }
                Ok((text, cursor + 1))
            }
            Encoding::UTF16_LE => {
                while cursor < payload.len() && payload[cursor] != 0 && payload[cursor + 1] != 0 {
                    text_vec.push(payload[cursor]);
                    cursor += 1;
                }
                text = String::from_utf16(&util::into_big_endian_u16(&text_vec, true)?).expect("");
                if text.len() == 0 {
                    text = "null".to_string();
                }
                Ok((text, cursor + 2))
            }
            Encoding::UTF16_BE => {
                while cursor < payload.len() && payload[cursor] != 0 && payload[cursor + 1] != 0 {
                    text_vec.push(payload[cursor]);
                    cursor += 1;
                }
                text = String::from_utf16(&util::into_big_endian_u16(&text_vec, false)?).expect("");
                if text.len() == 0 {
                    text = "null".to_string();
                }
                Ok((text, cursor + 2))
            }
            Encoding::UTF8 => {
                while cursor < payload.len() && payload[cursor] != 0 {
                    text_vec.push(payload[cursor]);
                    cursor += 1;
                }
                text = String::from_utf8(text_vec).expect("");
                if text.len() == 0 {
                    text = "null".to_string();
                }
                Ok((text, cursor + 1))
            }
            _ => Err(HeaderError::UnknownError(
                "UTF16_WITH_BOM is not allowed".to_string(),
            )),
        }
    }

    pub fn refine_encoding(payload: &[u8]) -> Encoding {
        if payload[0] == 0xFF && payload[1] == 0xFE {
            return Encoding::UTF16_LE;
        } else {
            return Encoding::UTF16_BE;
        }
    }
}
