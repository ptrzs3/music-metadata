# Music-metadata

Music Metadata Parser for Developer

# Introduction

## Supports

✔️ID3v2.3

✔️ID3v2.4

✔️ID3v1(.1)

Developing other music formats such as flac, ogg, etc.


## Usage

```shell
git clone https://github.com/ptrzs3/music-metadata.git
cd ./music-metadata
cargo run --example runme
```

## Example

```rust
use music_metadata::ID3_Parser;
fn main() -> std::io::Result<()> {
    let mut parser = ID3_Parser::new("云烟成雨.mp3").unwrap();
    
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

```

## License

Apache-2.0 License. See [LICENSE](https://github.com/ptrzs3/music-metadata/blob/main/LICENSE) file for details.

## Author

[ptrzs3](https://github.com/ptrzs3)

