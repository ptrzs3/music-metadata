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
```

## License

Apache-2.0 License. See [LICENSE](https://github.com/ptrzs3/music-metadata/blob/main/LICENSE) file for details.

## Author

[ptrzs3](https://github.com/ptrzs3)

