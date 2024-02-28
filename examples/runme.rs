use music_metadata::{FlacParser, ID3Parser, OggParser};
fn main() -> std::io::Result<()> {
    // https://drive.google.com/file/d/1fp_TYclIKZAWMwFTnxEEe4PqJCuBqHl4/view?usp=sharing
    let mut id3_parser = ID3Parser::new("云烟成雨.mp3").unwrap();

    id3_parser.parse_id3v1()?;
    // The ID3v1 protocol does not specify the content encoding,
    // which may be UTF-8, GBK or some other encoding,
    // so it is not possible to decode it, so the bytecode is output directly
    println!("{}", id3_parser.id3v1);

    id3_parser.parse_id3v2()?;
    println!("{}", id3_parser.pheader);
    println!("{}", id3_parser.eheader);
    println!("{}", id3_parser.footer);

    // The `get` method is case insensitive,
    // so you're allowed to pass in uppercase or lowercase characters or a mix of upper and lowercase characters,
    // as is the `get_raw` method
    println!("TIT2 = {:?}", id3_parser.get("TIT2").unwrap());
    println!("TALB = {:?}", id3_parser.get("talb").unwrap());
    println!("TPE1 = {:?}", id3_parser.get("tpe1").unwrap());
    println!("TPE2 = {:?}", id3_parser.get("tpe2").unwrap());
    println!("padding size = {}", id3_parser.padding_size);

    // It is not recommended to print the APIC byte sequence because it is really long
    // println!("APIC_raw = {:?}", parser.get_raw("apic").unwrap());

    // Write filename.jpg to the current directory.
    // No need to worry about multiple APIC frames in a file being overwritten by the same name.
    // Naming rules: <filename>_mp3_<picture_type>[_index].jpg
    id3_parser.write_image()?;

    let mut flac_parser = FlacParser::new("云烟成雨.flac").unwrap();
    flac_parser.parse()?;

    // https://www.xiph.org/vorbis/doc/v-comment.html
    // The `get` method is case insensitive
    println!("artist = {:?}", flac_parser.get("artist").unwrap());

    println!("album = {:?}", flac_parser.get("album").unwrap());

    // Get all vorbis comments in the file
    let (k, v) = flac_parser.get_all().unwrap();
    let mut index = 0;
    while index < k.len() {
        println!(
            "vorbis key = {:?}, vorbis comment = {:?}",
            k[index], v[index]
        );
        index += 1;
    }

    // It is not recommended to print the APIC byte sequence because it is really long
    // println!("picture_raw_data = {:?}", flac_parser.picture[0].data);

    println!("md5 = {}", flac_parser.stream_info.md5);

    println!(
        "picture width = {}, picture width = {}, picture type = {:?}",
        flac_parser.picture[0].width,
        flac_parser.picture[0].height,
        flac_parser.picture[0].pic_type
    );

    // This will write image[s] to disk
    // Naming rules: <filename>_flac_<picture_type>[_index].jpg
    flac_parser.write_image()?;

    flac_parser.change_target("千千阙歌.flac");


    let mut ogg_parser = OggParser::new("xhh.ogg");
    ogg_parser.parse()?;
    println!("ogg_vorbis_comment = {:?}", ogg_parser.get_all());
    Ok(())
}
