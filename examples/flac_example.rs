use music_metadata::FlacParser;
fn main() -> std::io::Result<()> {
    let mut parser = FlacParser::new(r"/Users/zhangsan/Downloads/云烟成雨.flac")?;
    parser.parse()?;
    println!("parser = {:?}", parser);
    Ok(())
}
