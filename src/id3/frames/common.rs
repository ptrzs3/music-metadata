#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Encoding {
    ISO_8859_1,
    UTF16_LE,
    UTF16_WITH_BOM,
    UTF16_BE,
    UTF8,
}

pub trait Tape {
    // 有些frame不止出现一次
    fn identifier(&self) -> String;
    fn message(&self) -> String;
    fn raw(&self) -> Vec<u8>;
}