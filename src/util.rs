use std::collections::VecDeque;

use crate::id3::{error::header_error::HeaderError, frames::header::Byte};
pub type Buffer = Vec<u8>;

pub fn map_to_binary(decimal: &[u8]) -> Vec<Byte> {
    let mut result: Vec<Byte> = Vec::new();
    for value in decimal.iter() {
        let mut byte: Byte = Vec::new();
        for i in (0..=7).rev() {
            byte.push(value >> i & 1);
        }
        result.push(byte);
    }
    result
}


pub fn into_big_endian_u16(text: &[u8], reverse: bool) -> Result<Vec<u16>, HeaderError> {
    let mut big_endian_u16: Vec<u16> = Vec::new();
    match reverse {
        true => {
            for index in 0..text.len() / 2 {
                let high_u8 = text[index * 2 + 1];
                let low_u8 = text[index * 2];
                big_endian_u16.push(((high_u8 as u16) << 8) + low_u8 as u16);
            }
        }
        false => {
            for index in 0..text.len() / 2 {
                let high_u8 = text[index * 2];
                let low_u8 = text[index * 2 + 1];
                big_endian_u16.push(((high_u8 as u16) << 8) + low_u8 as u16);
            }
        }
    }
    // 将结尾的0x00删除
    while let Some(0x00) = big_endian_u16.last() {
        big_endian_u16.pop();
    }
    Ok(big_endian_u16)
}

pub fn latin1_to_string(latin1: &[u8]) -> String {
    // let mut latin1_owned: Vec<u8> = latin1.into();
    // 使用MP3tag修改信息时，会在信息结尾加$00(00)，而ID3协议中不应该有$00(00)
    // 保险起见，将结尾的0x00删除
    let mut vq: VecDeque<u8> = VecDeque::from(Into::<Vec<u8>>::into(latin1));
    while let Some(0x00) = vq.front() {
        vq.pop_front();
    }
    while let Some(0x00) = vq.back() {
        vq.pop_back();
    }
    vq.iter().map(|&c| c as char).collect()
}
