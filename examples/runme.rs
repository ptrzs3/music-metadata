use music_metadata::Parser;
fn main() -> std::io::Result<()> {
    let mut parser  = Parser::new("云烟成雨.mp3").unwrap();
    parser.parse_id3v1()?;
    println!("{:?}", parser.id3v1);
    parser.parse_id3v2()?;
    println!("protocol header = {:?}", parser.pheader);
    println!("extended header = {:?}", parser.eheader);
    println!("TIT2 = {:?}", parser.get("TIT2").unwrap());
    
    println!("TALB = {:?}", parser.get("talb").unwrap());
    // It is not recommended to print the APIC byte sequence because it is really long
    // println!("APIC_raw = {:?}", parser.get_raw("apic").unwrap());
    parser.write_image()?;
    Ok(())
}