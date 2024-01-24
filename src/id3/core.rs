use crate::util::{self, Buffer};

use super::{
    common, extended_header::ExtendedHeader, footer::Footer, frames::{
        common::{Encoding, Tape},
        header::FrameHeader,
        identifiers::{
            IDFactory, RarelyUsedFrameIdentifier, TextInformationFrameIdentifier,
            URLLinkFrameIdentifier,
        },
        rarely_used::RarelyUsed,
        text_infomation_frame::TextInfomationFrame,
        url_link_frame::URLLinkFrame,
        APIC::{PicType, APIC},
        COMM::COMM,
        SYLT::SYLT,
        TXXX::TXXX,
        USLT::USLT,
        WXXX::WXXX,
    }, error::ID3Error, protocol_header::{Flag, ProtocolHeader}, version::Version
};

pub fn parse_protocol_header(header: &Buffer) -> Result<ProtocolHeader, ID3Error> {
    let protocol_header = ProtocolHeader {
        identifier: String::from_utf8_lossy(&header[..=2]).into_owned(),
        major_version: {
            if header[0..=3] == [0x49, 0x44, 0x33, 0x03]
                || header[0..=3] == [0x33, 0x44, 0x49, 0x03]
            {
                Version::V3
            } else if header[0..=3] == [0x49, 0x44, 0x33, 0x04]
                || header[0..=3] == [0x33, 0x44, 0x49, 0x04]
            {
                Version::V4
            } else {
                return Err(ID3Error::Unimplement("Wrong Header".to_string(), 1));
            }
        },
        revision: header[4],
        flags: Flag::new(header[5]),
        size: common::get_size(header[6..].to_vec(), &Version::V4),
    };
    Ok(protocol_header)
}

pub fn parse_footer_buffer(footer: &Buffer) -> Result<Footer, ID3Error> {
    Ok(Footer::from(parse_protocol_header(footer)?))
}

pub fn parse_extended_header(header: &Buffer, version: &Version) -> ExtendedHeader {
    ExtendedHeader::new(
        version.clone(),
        common::get_size(header[0..=3].to_vec(), version) as u8,
        header[4..].into(),
    )
}
pub fn parse_frame_header(header: &Buffer, version: &Version) -> Result<FrameHeader, ID3Error> {
    let frame_header = FrameHeader {
        identifier: IDFactory::from(header[0..=3].to_vec()),
        size: common::get_size(header[4..8].to_vec(), version),
        flags: util::map_to_binary(&header[8..]),
        version: version.clone(),
    };
    if let IDFactory::R(RarelyUsedFrameIdentifier::UNIMPLEMENT(id)) = frame_header.identifier {
        return Err(ID3Error::Unimplement(id, frame_header.size));
    }
    if let IDFactory::PADDING = frame_header.identifier {
        return Err(ID3Error::IsPadding);
    }
    Ok(frame_header)
}

pub fn parse_frame_payload(
    payload: &Buffer,
    header: &FrameHeader,
) -> Result<Box<dyn Tape>, ID3Error> {
    match &header.identifier {
        IDFactory::T(id) => {
            if let TextInformationFrameIdentifier::TXXX = id {
                let txxx = parse_TXXX(payload.clone())?;
                Ok(Box::new(txxx))
            } else {
                let text_infomation_frame =
                    parse_text_infomation_frame(header.identifier.to_string(), payload.clone())?;
                Ok(Box::new(text_infomation_frame))
            }
        }
        IDFactory::W(id) => {
            if let URLLinkFrameIdentifier::WXXX = id {
                let wxxx = parse_WXXX(payload.clone())?;
                Ok(Box::new(wxxx))
            } else {
                let url_link_frame =
                    parse_url_link_frame(header.identifier.to_string(), payload.clone())?;
                Ok(Box::new(url_link_frame))
            }
        }
        IDFactory::APIC => {
            let apic = parse_APIC(payload.clone())?;
            Ok(Box::new(apic))
        }
        IDFactory::COMM => {
            let comm = parse_COMM(payload.clone())?;
            Ok(Box::new(comm))
        }
        IDFactory::USLT => {
            let uslt = parse_USLT(payload.clone())?;
            Ok(Box::new(uslt))
        }
        IDFactory::SYLT => {
            let sylt = parse_SYLT(payload.clone())?;
            Ok(Box::new(sylt))
        }
        IDFactory::R(_) => {
            let rarely_used = parse_RarelyUsed(header.identifier.to_string(), payload.clone())?;
            Ok(Box::new(rarely_used))
        }
        _ => {
            panic!("Unexpected Error")
        }
    }
}

#[allow(non_snake_case)]
fn parse_text_infomation_frame(
    identifier: String,
    payload: Buffer,
) -> Result<TextInfomationFrame, ID3Error> {
    let mut encoding = common::get_encoding(payload[0])?;
    let mut cursor: usize = 1;
    if let Encoding::UTF16_WITH_BOM = encoding {
        encoding = common::refine_encoding(&payload[1..=2]);
        cursor += 2;
    }
    let data = common::get_text(&encoding, &payload[cursor..])?;
    Ok(TextInfomationFrame::new(identifier, data))
}

fn parse_url_link_frame(
    identifier: String,
    payload: Buffer,
) -> Result<URLLinkFrame, ID3Error> {
    let data = common::get_text(&Encoding::UTF8, &payload[..])?;
    Ok(URLLinkFrame::new(identifier, data))
}

#[allow(non_snake_case)]
fn parse_TXXX(payload: Buffer) -> Result<TXXX, ID3Error> {
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
fn parse_WXXX(payload: Buffer) -> Result<WXXX, ID3Error> {
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
fn parse_USLT(payload: Buffer) -> Result<USLT, ID3Error> {
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
    Ok(USLT::new(data_encoding, language, description, data))
}

#[allow(non_snake_case)]
fn parse_SYLT(payload: Buffer) -> Result<SYLT, ID3Error> {
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
fn parse_COMM(payload: Buffer) -> Result<COMM, ID3Error> {
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
fn parse_APIC(payload: Buffer) -> Result<APIC, ID3Error> {
    let mut encoding = common::get_encoding(payload[0])?;
    let mut cursor: usize = 1;
    let (MIME_type, skip): (String, usize) =
        common::get_text_according_to_encoding(&payload[cursor..], &Encoding::UTF8)?;
    cursor += skip;
    let raw_pic_type = payload[cursor];
    let picture_type: PicType = PicType::from(payload[cursor]);
    cursor += 1;
    if let Encoding::UTF16_WITH_BOM = encoding {
        encoding = common::refine_encoding(&payload[cursor..=cursor + 1]);
        cursor += 2;
    }
    let (description, skip): (String, usize) =
        common::get_text_according_to_encoding(&payload[cursor..], &encoding)?;
    cursor += skip;
    let mut data: Vec<u8> = payload[cursor..].into();
    data.push(raw_pic_type);
    Ok(APIC::new(
        encoding,
        MIME_type,
        picture_type,
        description,
        data,
    ))
}

#[allow(non_snake_case)]
fn parse_RarelyUsed(identifier: String, payload: Buffer) -> Result<RarelyUsed, ID3Error> {
    Ok(RarelyUsed::new(identifier, payload))
}
