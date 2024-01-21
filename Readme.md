# Music-metadata

Music Metadata Parser for Developer

# Introduction

## Supports

✔️ID3v2.3

✔️ID3v2.4

❌ID3v1 (Soon)

Developing other music formats such as flac, ogg, etc.



## Example

```rust
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
```

## License

Apache License 2.0. See [LICENSE](https://github.com/ptrzs3/music-metadata/blob/main/LICENSE) file for details.

## Author

[ptrzs3](https://github.com/ptrzs3)

