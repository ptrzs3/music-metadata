pub trait Light {
    fn get_identifier(self) -> String;
    fn get_data(self) -> String;
}

pub trait Heavy {
    fn get_identifier(self) -> String;
    fn get_raw_data(self) -> Vec<u8>;
    fn get_addition(self) -> String;
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Encoding {
    ISO_8859_1,
    UTF16_LE,
    UTF16_WITH_BOM,
    UTF16_BE,
    UTF8,
}
