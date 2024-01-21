use music_metadata::Parser;
fn main() -> std::io::Result<()> {
    let mut parser  = Parser::new("云烟成雨.mp3");
    parser.parse_file()?;
    let tit2: Vec<String> = parser.get("TIT2").unwrap();
    println!("{:?}", tit2);
    let raw_apic: &Vec<u8> = &parser.get_raw("APIC").unwrap()[0];
    println!("{:?}", raw_apic);
    parser.write_image()?;
    Ok(())
}