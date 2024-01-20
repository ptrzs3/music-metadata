use std::{fmt::Display, fs, io, path::Path};

use super::common::Encoding;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[derive(Debug)]
pub struct APIC {
    identifier: String,
    encoding: Encoding,
    MIME_type: String,
    picture_type: PicType,
    description: String,
    data: Vec<u8>,
}

impl APIC {
    #[allow(non_snake_case)]
    pub fn new(
        encoding: Encoding,
        MIME_type: String,
        picture_type: PicType,
        description: String,
        data: Vec<u8>,
    ) -> Self {
        APIC {
            identifier: "APIC".to_string(),
            encoding,
            MIME_type,
            picture_type,
            description,
            data,
        }
    }
    
    #[allow(dead_code)]
    pub fn write<T: AsRef<Path>>(&self, path: T) -> io::Result<()> {
        fs::write(path, &self.data)
    }
}

impl Display for APIC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
APIC {{
    encoding: {:?},
    MIME_type: {},
    picture_type: {:?},
    description: {},
    data: {:X?}
}}",
            self.encoding, self.MIME_type, self.picture_type, self.description, self.data[0]
        )
    }
}

// impl Heavy for APIC {
//     fn get_addition(self) -> String {
//         format!(
//             "MIME: {}, type: {:?}, description: {}",
//             self.MIME_type, self.picture_type, self.description
//         )
//     }
//     fn get_raw_data(self) -> Vec<u8> {
//         self.data
//     }
//     fn get_identifier(self) -> String {
//         "APIC".to_string()
//     }
// }

#[derive(Debug)]
pub enum PicType {
    Other,
    FileIcon32x32,
    OtherFileIcon,
    FrontCover,
    BackCover,
    LeafletPage,
    Media,
    LeadArtist,
    ArtistOrPerformer,
    Conductor,
    BandOrOrchestra,
    Composer,
    LyricistWriter,
    RecordingLocation,
    DuringRecording,
    DuringPerformence,
    MovieOrVideoScreenCapture,
    ABrightColouredFish,
    Illustration,
    BandOrArtistLogotype,
    PublisherOrStudioLogoType,
}

impl From<u8> for PicType {
    fn from(value: u8) -> Self {
        match value {
            0x00 => PicType::Other,
            0x01 => PicType::FileIcon32x32,
            0x02 => PicType::OtherFileIcon,
            0x03 => PicType::FrontCover,
            0x04 => PicType::BackCover,
            0x05 => PicType::LeafletPage,
            0x06 => PicType::Media,
            0x07 => PicType::LeadArtist,
            0x08 => PicType::ArtistOrPerformer,
            0x09 => PicType::Conductor,
            0x0A => PicType::BandOrOrchestra,
            0x0B => PicType::Composer,
            0x0C => PicType::LyricistWriter,
            0x0D => PicType::RecordingLocation,
            0x0E => PicType::DuringRecording,
            0x0F => PicType::DuringPerformence,
            0x10 => PicType::MovieOrVideoScreenCapture,
            0x11 => PicType::ABrightColouredFish,
            0x12 => PicType::Illustration,
            0x13 => PicType::BandOrArtistLogotype,
            0x14 => PicType::PublisherOrStudioLogoType,
            _ => panic!("Invalid value for conversion to PicType"),
        }
    }
}
