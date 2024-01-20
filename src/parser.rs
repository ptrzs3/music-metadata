use crate::error::frame_error::FrameError;
use crate::frames::identifiers::{
    IDFactory, RarelyUsedFrameIdentifier, TextInformationFrameIdentifier, URLLinkFrameIdentifier,
};
use crate::frames::rarely_used::RarelyUsed;
use crate::frames::text_infomation_frame::TextInfomationFrame;
use crate::frames::url_link_frame::URLLinkFrame;
use crate::frames::APIC::{PicType, APIC};
use crate::frames::COMM::COMM;
use crate::frames::SYLT::SYLT;
use crate::frames::TXXX::TXXX;
use crate::frames::USLT::USLT;
use crate::frames::WXXX::WXXX;
use crate::frames::{common::Encoding, header::FrameHeader};
use crate::protocol_header::ProtocolHeader;
use crate::reader::Buffer;
use crate::util;
use crate::version::Version;
use std::io;

use self::common::{get_text, get_text_according_to_encoding};

pub fn parse_protocol_header(header: &Buffer) -> io::Result<ProtocolHeader> {
    let header = ProtocolHeader {
        identifier: String::from_utf8_lossy(&header[..3]).into_owned(),
        major_version: {
            if header[3] == 3 {
                Version::V3
            } else {
                Version::V4
            }
        },
        revision: header[4],
        size: util::get_size(header[6..].to_vec(), &Version::V4),
        flags: util::map_to_binary(&[header[5]])[0].clone(),
    };
    Ok(header)
}

pub fn parse_frame_header(header: &Buffer, version: &Version) -> Result<FrameHeader, FrameError> {
    let frame_header = FrameHeader {
        identifier: IDFactory::from(header[0..=3].to_vec()),
        size: util::get_size(header[4..8].to_vec(), version),
        flags: util::map_to_binary(&header[8..]),
        version: version.clone(),
    };
    if let IDFactory::R(RarelyUsedFrameIdentifier::UNIMPLEMENT(id)) = frame_header.identifier {
        return Err(FrameError::Unimplement(id, frame_header.size));
    }
    if let IDFactory::PADDING = frame_header.identifier {
        return Err(FrameError::IsPadding);
    }
    Ok(frame_header)
}

pub fn parse_frame_payload(payload: &Buffer, header: &FrameHeader) -> Result<(), FrameError> {
    let _ = match &header.identifier {
        IDFactory::T(id) => {
            if let TextInformationFrameIdentifier::TXXX = id {
                let txxx = parse_TXXX(payload.clone())?;
                println!("{}", txxx)
            } else {
                let text_infomation_frame =
                    parse_text_infomation_frame(header.identifier.to_string(), payload.clone())?;
                println!("{}", text_infomation_frame);
            }
        }
        IDFactory::W(id) => {
            if let URLLinkFrameIdentifier::WXXX = id {
                let wxxx = parse_WXXX(payload.clone())?;
                println!("{}", wxxx);
            } else {
                let url_link_frame =
                    parse_url_link_frame(header.identifier.to_string(), payload.clone())?;
                println!("{}", url_link_frame);
            }
        }
        IDFactory::APIC => {
            let apic = parse_APIC(payload.clone())?;
            println!("{}", apic);
        }
        IDFactory::COMM => {
            let comm = parse_COMM(payload.clone())?;
            println!("{}", comm);
        }
        IDFactory::USLT => {
            let uslt = parse_USLT(payload.clone())?;
            println!("{}", uslt);
        }
        IDFactory::SYLT => {
            let sylt = parse_SYLT(payload.clone())?;
            println!("{}", sylt);
        }
        IDFactory::R(_) => {
            let rarely_used = parse_RarelyUsed(header.identifier.to_string(), payload.clone())?;
            println!("{}", rarely_used);
        }
        _ => {
            panic!("Unexpected Error")
        }
    };
    Ok(())
}

#[allow(non_snake_case)]
fn parse_text_infomation_frame(
    identifier: String,
    payload: Buffer,
) -> Result<TextInfomationFrame, FrameError> {
    let mut encoding = common::get_encoding(payload[0])?;
    let mut cursor: usize = 1;
    if let Encoding::UTF16_WITH_BOM = encoding {
        encoding = common::refine_encoding(&payload[1..=2]);
        cursor += 2;
    }
    let data = get_text(&encoding, &payload[cursor..])?;
    Ok(TextInfomationFrame::new(identifier, data))
}

fn parse_url_link_frame(identifier: String, payload: Buffer) -> Result<URLLinkFrame, FrameError> {
    let data = get_text(&Encoding::UTF8, &payload[..])?;
    Ok(URLLinkFrame::new(identifier, data))
}

#[allow(non_snake_case)]
fn parse_TXXX(payload: Buffer) -> Result<TXXX, FrameError> {
    let mut encoding = common::get_encoding(payload[0])?;
    let mut cursor = 1;
    if let Encoding::UTF16_WITH_BOM = encoding {
        encoding = common::refine_encoding(&payload[cursor..=cursor + 1]);
        cursor += 2;
    }
    let (description, skip) = get_text_according_to_encoding(&payload[cursor..], &encoding)?;
    cursor += skip;
    let data = get_text(&Encoding::UTF8, &payload[cursor..])?;
    Ok(TXXX::new(encoding, description, data))
}

#[allow(non_snake_case)]
fn parse_WXXX(payload: Buffer) -> Result<WXXX, FrameError> {
    let mut encoding = common::get_encoding(payload[0])?;
    let mut cursor = 1;
    if let Encoding::UTF16_WITH_BOM = encoding {
        encoding = common::refine_encoding(&payload[cursor..=cursor + 1]);
        cursor += 2;
    }
    let (description, skip) = get_text_according_to_encoding(&payload[cursor..], &encoding)?;
    cursor += skip;
    let data = get_text(&Encoding::UTF8, &payload[cursor..])?;
    Ok(WXXX::new(encoding, description, data))
}

#[allow(non_snake_case)]
fn parse_USLT(payload: Buffer) -> Result<USLT, FrameError> {
    let frame_encoding = common::get_encoding(payload[0])?;
    let mut data_encoding = common::get_encoding(payload[0])?;
    let language: String = String::from_utf8(payload[1..=3].into()).expect("");
    let mut cursor: usize = 4;
    if let Encoding::UTF16_WITH_BOM = frame_encoding {
        data_encoding = common::refine_encoding(&payload[cursor..=cursor + 1]);
        cursor += 2;
    }
    let (descriptor, skip): (String, usize) =
        common::get_text_according_to_encoding(&payload[cursor..], &data_encoding)?;
    cursor += skip;
    if let Encoding::UTF16_WITH_BOM = frame_encoding {
        data_encoding = common::refine_encoding(&payload[cursor..=cursor + 1]);
        cursor += 2;
    }
    let data = common::get_text(&data_encoding, &payload[cursor..])?;
    Ok(USLT::new(data_encoding, language, descriptor, data.into()))
}

#[allow(non_snake_case)]
fn parse_SYLT(payload: Buffer) -> Result<SYLT, FrameError> {
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
fn parse_COMM(payload: Buffer) -> Result<COMM, FrameError> {
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
fn parse_APIC(payload: Buffer) -> Result<APIC, FrameError> {
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
fn parse_RarelyUsed(identifier: String, payload: Buffer) -> Result<RarelyUsed, FrameError> {
    Ok(RarelyUsed::new(identifier, payload))
}
mod common {

    use std::collections::VecDeque;

    use crate::{error::frame_error::FrameError, frames::common::Encoding, util};

    pub fn get_text(encoding: &Encoding, payload: &[u8]) -> Result<String, FrameError> {
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

    pub fn get_encoding(payload: u8) -> Result<Encoding, FrameError> {
        let encoding = match payload {
            0x00 => Encoding::ISO_8859_1,
            0x01 => Encoding::UTF16_WITH_BOM,
            0x02 => Encoding::UTF16_BE,
            0x03 => Encoding::UTF8,
            _ => {
                return Err(FrameError::UnknownError(
                    "Out-of-bounds indexing".to_string(),
                ))
            }
        };
        Ok(encoding)
    }

    pub fn get_text_according_to_encoding(
        payload: &[u8],
        encoding: &Encoding,
    ) -> Result<(String, usize), FrameError> {
        let mut cursor: usize = 0;
        let mut text_vec: Vec<u8> = Vec::new();
        let text: String;
        match encoding {
            Encoding::ISO_8859_1 => {
                while cursor < payload.len() && payload[cursor] != 0 {
                    text_vec.push(payload[cursor]);
                    cursor += 1;
                }
                text = util::latin1_to_string(&text_vec);
                Ok((text, cursor + 1))
            }
            Encoding::UTF16_LE => {
                while cursor < payload.len() && payload[cursor] != 0 && payload[cursor + 1] != 0 {
                    text_vec.push(payload[cursor]);
                    cursor += 1;
                }
                text = String::from_utf16(&util::into_big_endian_u16(&text_vec, true)?).expect("");
                Ok((text, cursor + 2))
            }
            Encoding::UTF16_BE => {
                while cursor < payload.len() && payload[cursor] != 0 && payload[cursor + 1] != 0 {
                    text_vec.push(payload[cursor]);
                    cursor += 1;
                }
                text = String::from_utf16(&util::into_big_endian_u16(&text_vec, false)?).expect("");
                Ok((text, cursor + 2))
            }
            Encoding::UTF8 => {
                while cursor < payload.len() && payload[cursor] != 0 {
                    text_vec.push(payload[cursor]);
                    cursor += 1;
                }
                text = String::from_utf8(text_vec).expect("");
                Ok((text, cursor + 1))
            }
            _ => Err(FrameError::UnknownError(
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
