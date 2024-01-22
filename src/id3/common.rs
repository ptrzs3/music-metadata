use std::collections::VecDeque;

use crate::{
    id3::{error::header_error::HeaderError, frames::common::Encoding, version::Version},
    util,
};

// use super::version::Version;

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
            if text.is_empty() {
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
            if text.is_empty() {
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
            if text.is_empty() {
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
            if text.is_empty() {
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
        Encoding::UTF16_LE
    } else {
        Encoding::UTF16_BE
    }
}

pub fn get_size(size: Vec<u8>, v: &Version) -> u32 {
    match v {
        Version::V3 => {
            size[0] as u32 * 0x1000000
                + size[1] as u32 * 0x10000
                + size[2] as u32 * 0x100
                + size[3] as u32
        }
        Version::V4 => {
            (size[0] as u32 & 0x7F) * 0x200000
                + (size[1] as u32 & 0x7F) * 0x4000
                + (size[2] as u32 & 0x7F) * 0x80
                + (size[3] as u32 & 0x7F)
        }
        Version::Default => 0,
    }
}
