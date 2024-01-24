use music_metadata::FlacParser;
fn main() -> std::io::Result<()> {
    let mut parser = FlacParser::new(r"C:\Users\ptrzs\Desktop\q\mix.flac")?;
    parser.parse()?;
    println!("parser = {:?}", parser);
    Ok(())
}
