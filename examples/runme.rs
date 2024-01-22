use music_metadata::Parser;
fn main() -> std::io::Result<()> {
    let mut parser = Parser::new("CS.mp3").unwrap();
    parser.parse_id3v1()?;
    println!("{}", parser.id3v1);
    parser.parse_id3v2()?;
    println!("{}", parser.pheader);
    println!("{}", parser.eheader);
    println!("padding size = {}", parser.padding_size);
    println!("{}", parser.footer);
    println!("TIT2 = {:?}", parser.get("TIT2").unwrap());
    println!("TALB = {:?}", parser.get("talb").unwrap());
    println!("TPE1 = {:?}", parser.get("tpe1").unwrap());
    // It is not recommended to print the APIC byte sequence because it is really long
    // println!("APIC_raw = {:?}", parser.get_raw("apic").unwrap());
    // parser.write_image()?;
    Ok(())
}
