use music_metadata::FlacParser;
fn main() -> std::io::Result<()> {
    let mut parser = FlacParser::new(r"C:\Users\ptrzs\Desktop\q\kanyewest.flac")?;
    parser.parse()?;
    // println!("parser = {:?}", parser.vorbis_comment);
    println!("parser.picture = {:?}", parser.picture.len());
    parser.write_image()?;
    
    Ok(())
}
