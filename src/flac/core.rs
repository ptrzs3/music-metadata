use std::io;

use crate::{
    flac::{blocks::block_picture::PicType, error::FlacError},
    util::{
        parse_4_bytes_with_big_endian, parse_4_bytes_with_little_endian,
        parse_8_bytes_with_big_endian,
    },
};

use super::blocks::{
    block_application::BlockApplication,
    block_cue_sheet::{BlockCueSheet, Track, TrackIndex},
    block_header::{BlockHeader, BlockType},
    block_picture::BlockPicture,
    block_seektable::{BlockSeekTable, SeekPoint},
    block_stream_info::BlockStreamInfo,
    block_vorbis_comment::BlockVorbisComment,
};

pub fn parse_flac_marker(buffer: &Vec<u8>) -> Result<(), FlacError> {
    if buffer[..] == [0x66, 0x4C, 0x61, 0x43] {
        return Ok(());
    }
    Err(FlacError::WrongHeader)
}
pub fn parse_block_header(buffer: &Vec<u8>) -> io::Result<BlockHeader> {
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

pub fn parse_stream_info_block(buffer: &Vec<u8>) -> io::Result<BlockStreamInfo> {
    let min_block_size: u16 = buffer[0] as u16 * 0x100 + buffer[1] as u16; // 2 Bytes
    let max_block_size: u16 = buffer[2] as u16 * 0x100 + buffer[3] as u16; // 2 Bytes
    let min_frame_size: u32 =
        buffer[4] as u32 * 0x10000 + buffer[5] as u32 * 0x100 + buffer[6] as u32; // 3 Bytes
    let max_frame_size: u32 =
        buffer[7] as u32 * 0x10000 + buffer[8] as u32 * 0x100 + buffer[9] as u32; // 3Bytes

    let sample_rate: u32 =
        buffer[10] as u32 * 0x1000 + buffer[11] as u32 * 0x10 + ((buffer[12] & 0xF0) >> 4) as u32; // 2 Bytes + 4 bits
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
    Ok(BlockStreamInfo::new(
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
pub fn parse_vorbis_comment(buf: &Vec<u8>) -> io::Result<BlockVorbisComment> {
    let mut vorbis_comment = BlockVorbisComment::default();
    let buffer: Vec<u8> = buf.to_owned();
    let mut start = 0;
    let mut end = 3;
    let encoder_length = parse_4_bytes_with_little_endian(&buffer[start..=end]);

    start = end + 1;
    end = start - 1 + encoder_length as usize;
    let encoder: String = String::from_utf8(buffer[start..=end].to_vec()).unwrap();
    vorbis_comment.encoder = encoder;
    start = end + 1;
    end = start - 1 + 4;
    let tags_number = parse_4_bytes_with_little_endian(&buffer[start..=end]);

    let mut tag_index = 0;

    while tag_index < tags_number {
        tag_index += 1;
        start = end + 1;
        end = start - 1 + 4;
        let tag_length = parse_4_bytes_with_little_endian(&buffer[start..=end]);
        start = end + 1;
        end = start - 1 + tag_length as usize;
        let tag_content_raw = String::from_utf8(buffer[start..=end].to_vec()).unwrap();
        let kv: Vec<&str> = tag_content_raw.split("=").collect();
        let tag_key: String = kv[0].to_owned();
        let tag_value = kv[1].to_owned();
        if let Some(index) = vorbis_comment.key_hash.get(&tag_key) {
            vorbis_comment.comment[*index].push(tag_value);
        } else {
            let tag_index = vorbis_comment.comment.len();
            vorbis_comment.key_hash.insert(tag_key.to_uppercase(), tag_index);
            vorbis_comment.comment.push(Vec::default());
            vorbis_comment.comment[tag_index].push(tag_value);
        }
    }
    Ok(vorbis_comment)
}
pub fn parse_block_picture(buf: &Vec<u8>) -> io::Result<BlockPicture> {
    let buffer: Vec<u8> = buf.to_owned();
    let mut start ;
    let mut end = 3;
    // let raw_pic_type = parse_4_bytes_with_big_endian(&buffer[start..=end]);
    let raw_pic_type = buffer[end];
    let pic_type: PicType = PicType::from(raw_pic_type);
    start = end + 1;
    end = start - 1 + 4;
    let mime_length = parse_4_bytes_with_big_endian(&buffer[start..=end]);
    start = end + 1;
    end = start - 1 + mime_length as usize;
    let mime: String = String::from_utf8(buffer[start..=end].to_vec()).unwrap();

    start = end + 1;
    end = start - 1 + 4;
    let desc_length = parse_4_bytes_with_big_endian(&buffer[start..=end]);

    start = end + 1;
    end = start - 1 + desc_length as usize;
    let description: String = String::from_utf8(buffer[start..=end].to_vec()).unwrap();

    start = end + 1;
    end = start - 1 + 4;
    let width = parse_4_bytes_with_big_endian(&buffer[start..=end]);

    start = end + 1;
    end = start - 1 + 4;
    let height = parse_4_bytes_with_big_endian(&buffer[start..=end]);

    start = end + 1;
    end = start - 1 + 4;
    let bit_depth = parse_4_bytes_with_big_endian(&buffer[start..=end]);

    start = end + 1;
    end = start - 1 + 4;
    let index_color_number = parse_4_bytes_with_big_endian(&buffer[start..=end]);

    start = end + 1;
    end = start - 1 + 4;
    let size = parse_4_bytes_with_big_endian(&buffer[start..=end]);

    start = end + 1;
    let mut data: Vec<u8> = buffer[start..].to_vec();
    data.push(raw_pic_type);
    Ok(BlockPicture::new(
        pic_type,
        mime,
        description,
        width,
        height,
        bit_depth,
        index_color_number,
        data,
        size,
    ))
}
pub fn parse_block_application(buf: &Vec<u8>) -> io::Result<BlockApplication> {
    let id = parse_4_bytes_with_big_endian(&buf[0..=3]);
    let data: Vec<u8> = buf[4..].to_owned();
    Ok(BlockApplication::new(id, data))
}
pub fn parse_block_seektable(buf: &Vec<u8>) -> io::Result<BlockSeekTable> {
    let buffer = buf.to_owned();
    let mut start = 0;
    let mut end = 7; // for the first loop
    let length = buf.len();
    let mut seek_table: BlockSeekTable = BlockSeekTable::default();
    while end < length {
        let mut seek_point = SeekPoint::default();

        let snfs = parse_8_bytes_with_big_endian(&buffer[start..=end]);
        seek_point.sample_number_of_first_sample = snfs;
        if snfs == 0xFFFFFFFFFFFFFFFF {
            seek_table.seekpoints.push(seek_point);
            start = end + 1;
            end = start - 1 + 8; // for next loop
            continue;
        }
        start = end + 1;
        end = start - 1 + 8;
        let offset = parse_8_bytes_with_big_endian(&buffer[start..=end]);
        seek_point.offset = offset;

        start = end + 1;
        end = start - 1 + 2;
        let na = buffer[start] as u16 * 0x100 + buffer[end] as u16;
        seek_point.number_of_samples = na;

        seek_table.seekpoints.push(seek_point);

        start = end + 1;
        end = start - 1 + 8; // for next loop
    }
    Ok(seek_table)
}

pub fn parse_block_cue_sheet(buf: &Vec<u8>) -> io::Result<BlockCueSheet> {
    let buffer = buf.to_owned();
    let mut start: usize = 0;
    let mut end: usize = 127;
    let media_catalog = String::from_utf8(buffer[start..=end].to_vec()).unwrap();

    start = end + 1;
    end = start - 1 + 8;
    let lead_in_samples_number: u64 = parse_8_bytes_with_big_endian(&buffer[start..=end]);

    start = end + 1;
    end = start - 1 + 1;
    let is_cd: bool = ((buffer[start] & 0x80) >> 7) == 1;

    start = end + 1;
    end = start - 1 + 258;
    // skip 7 bits + 258 bytes

    start = end + 1;
    end = start - 1 + 1;
    let tracks_number: u8 = buffer[start];

    let mut tracks: Vec<Track> = Vec::default();

    let mut i = 0;
    while i < tracks_number {
        i += 1;
        // let mut track = Track::default();
        start = end + 1;
        end = start - 1 + 8;
        let offset: u64 = parse_8_bytes_with_big_endian(&buffer[start..=end]);

        start = end + 1;
        end = start - 1 + 1;
        let number = buffer[start];

        start = end + 1;
        end = start - 1 + 12;
        let isrc = String::from_utf8(buffer[start..=end].to_vec())
            .to_owned()
            .unwrap();

        start = end + 1;
        end = start - 1 + 1;
        let is_audio_track: bool = ((buffer[start] & 0x80) >> 7) == 0;

        let pre_emphasis: bool = ((buffer[start] & 0x40) >> 6) == 1;

        start = end + 1;
        end = start - 1 + 13;
        // skip 6 bits and 13 bytes

        start = end + 1;
        end = start - 1 + 1;
        let track_index_points_number: u8 = buffer[start];

        let mut j = 0;
        let mut track_indices: Vec<TrackIndex> = Vec::default();
        while j < track_index_points_number {
            j += 1;
            start = end + 1;
            end = start - 1 + 8;
            let index_offset = parse_8_bytes_with_big_endian(&buffer[start..=end]);

            start = end + 1;
            end = start - 1 + 1;
            let index_point_number = buffer[start];

            start = end + 1;
            end = start - 1 + 3;
            // skip 3 bytes
            track_indices.push(TrackIndex::new(index_offset, index_point_number))
        }

        tracks.push(Track::new(
            offset,
            number,
            isrc,
            is_audio_track,
            pre_emphasis,
            track_index_points_number,
            track_indices,
        ));
    }
    Ok(BlockCueSheet::new(
        media_catalog,
        lead_in_samples_number,
        is_cd,
        tracks_number,
        tracks,
    ))
}
