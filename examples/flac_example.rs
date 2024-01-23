use music_metadata::FlacParser;
fn main() -> std::io::Result<()> {
    let mut parser = FlacParser::new(r"C:\Users\ptrzs\Desktop\q\qqqg.flac")?;
    parser.parse()?;
    Ok(())
}
