#![warn(clippy::all, clippy::pedantic)]

use std::io;
mod error;
mod frames;
mod protocol_header;
mod reader;
mod util;
mod version;
mod parser;

fn main() -> io::Result<()> {
    let mut parser = parser::Parser::new("cry.mp3");
    parser.parse_file()?;
    parser.write_image()?;

    parser.change_target("屋顶.mp3");
    parser.parse_file()?;
    parser.write_image()?;

    // parser.change_target("一直很安静.mp3");
    // parser.parse_file()?;
    // parser.write_image()?;

    // parser.change_target("Cry.mp3");
    // parser.parse_file()?;
    // parser.write_image()?;
    Ok(())
}
