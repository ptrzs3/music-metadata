use music_metadata::ID3Parser;
fn main() -> std::io::Result<()> {
    // https://drive.google.com/file/d/1fp_TYclIKZAWMwFTnxEEe4PqJCuBqHl4/view?usp=sharing
    let mut parser = ID3Parser::new(r"C:\Users\ptrzs\Desktop\q\云烟成雨.mp3").unwrap();
    
    parser.parse_id3v1()?;
    // The ID3v1 protocol does not specify the content encoding, 
    // which may be UTF-8, GBK or some other encoding, 
    // so it is not possible to decode it, so the bytecode is output directly
    println!("{}", parser.id3v1);

    parser.parse_id3v2()?;
    println!("{}", parser.pheader);
    println!("{}", parser.eheader);
    println!("{}", parser.footer);

    // The `get` method is case insensitive,
    // so you're allowed to pass in uppercase or lowercase characters or a mix of upper and lowercase characters,
    // as is the `get_raw` method
    println!("TIT2 = {:?}", parser.get("TIT2").unwrap());
    println!("TALB = {:?}", parser.get("talb").unwrap());
    println!("TPE1 = {:?}", parser.get("tpe1").unwrap());
    println!("TPE2 = {:?}", parser.get("tpe2").unwrap());
    println!("padding size = {}", parser.padding_size);
    
    // It is not recommended to print the APIC byte sequence because it is really long
    // println!("APIC_raw = {:?}", parser.get_raw("apic").unwrap());

    // Write filename.jpg to the current directory.
    // No need to worry about multiple APIC frames in a file being overwritten by the same name.
    parser.write_image()?;
    Ok(())
}
