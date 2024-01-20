#![warn(clippy::all, clippy::pedantic)]

use std::{io, path::Path};
mod error;
mod frames;
mod protocol_header;
mod reader;
mod util;
mod version;
use reader::BufferReader;
mod parser;
use crate::reader::Buffer;
use error::frame_error::FrameError;
use parser::{parse_frame_header, parse_frame_payload, parse_protocol_header};

fn parse_file<T: AsRef<Path>>(file: T) -> io::Result<()> {
    let mut buffer_reader = BufferReader::new(file)?;

    let mut buffer: Buffer;

    buffer = buffer_reader.read_protocol_header_buffer()?;
    let protocol_header = parse_protocol_header(&buffer)?;
    println!("{}", protocol_header.to_string());

    let mut start: u32 = 0;
    while start < protocol_header.size {
        buffer = buffer_reader.read_frame_header_buffer()?;
        match parse_frame_header(&buffer, &protocol_header.major_version) {
            Ok(v) => {
                buffer = buffer_reader.read_frame_payload_buffer(v.size)?;
                if let Err(e) = parse_frame_payload(&buffer, &v) {
                    println!("{:?}", e);
                }
                start += 10 + v.size;
            }
            Err(e) => match e {
                FrameError::IsPadding => {
                    println!("### Endding ###");
                    return Ok(());
                }
                FrameError::Unimplement(id, skip) => {
                    let buf = buffer_reader.skip(skip)?;
                    start += 10 + skip;
                    println!(
                        "unimplement: {{
identifier: {},
raw: {:?}",
                        id, buf
                    );
                }
                FrameError::UnknownError(s) => {
                    println!("{s}");
                    println!("The parser is stopped");
                    return Ok(());
                }
            },
        }
    }
    Ok(())
}

fn main() {
    parse_file("云烟成雨.mp3").unwrap();
    parse_file("CS.mp3").unwrap();
    parse_file("单车.mp3").unwrap();
    parse_file("Mixotic.mp3").unwrap()
}
