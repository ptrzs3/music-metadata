pub struct PageHeader {
    pub capture_pattern: String,
    pub structure_version: u8,
    // pub header_type_flag: u8,
    pub new_packet: bool,
    pub bos: bool,
    pub eos: bool,
    pub granule_position: Vec<u8>,
    pub serial_number: Vec<u8>,
    pub page_sequence_number: Vec<u8>,
    pub crc_checksum: Vec<u8>,
    pub number_page_segments: u8,
    pub segment_table: Vec<u8>,
}
impl Default for PageHeader {
    fn default() -> Self {
        PageHeader {
            capture_pattern: String::default(),
            structure_version: u8::default(),
            // header_type_flag: u8::default(),
            new_packet:bool::default(),
            bos: bool::default(),
            eos: bool::default(),
            granule_position: Vec::default(),
            serial_number: Vec::default(),
            page_sequence_number: Vec::default(),
            crc_checksum: Vec::default(),
            number_page_segments: u8::default(),
            segment_table: Vec::default(),
        }
    }
}
