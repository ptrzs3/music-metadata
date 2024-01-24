#[derive(Debug)]
#[derive(Default)]
pub struct BlockPicture {
    pub pic_type: PicType,
    pub mime: String,
    pub description: String,
    pub width: u32,
    pub height: u32,
    pub bit_depth: u32,
    pub index_color_number: u32,
    pub data: Vec<u8>,
    pub size: u32,
}
impl BlockPicture {
    pub fn new(
        pic_type: PicType,
        mime: String,
        description: String,
        width: u32,
        height: u32,
        bit_depth: u32,
        index_color_number: u32,
        data: Vec<u8>,
        size: u32,
    ) -> Self {
        BlockPicture {
            pic_type,
            mime,
            description,
            width,
            height,
            bit_depth,
            index_color_number,
            data,
            size,
        }
    }
}



#[derive(Debug)]
#[derive(Default)]
pub enum PicType {
    #[default]
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
impl ToString for PicType {
    fn to_string(&self) -> String {
        match self {
            Self::Other => String::from("Other"),
            Self::FileIcon32x32 => String::from("FileIcon32x32"),
            Self::OtherFileIcon => String::from("OtherFileIcon"),
            Self::FrontCover => String::from("FrontCover"),
            Self::BackCover => String::from("BackCover"),
            Self::LeafletPage => String::from("LeafletPage"),
            Self::Media => String::from("Media"),
            Self::LeadArtist => String::from("LeadArtist"),
            Self::ArtistOrPerformer => String::from("ArtistOrPerformer"),
            Self::Conductor => String::from("Conductor"),
            Self::BandOrOrchestra => String::from("BandOrOrchestra"),
            Self::Composer => String::from("Composer"),
            Self::LyricistWriter => String::from("LyricistWriter"),
            Self::RecordingLocation => String::from("RecordingLocation"),
            Self::DuringRecording => String::from("DuringRecording"),
            Self::DuringPerformence => String::from("DuringPerformence"),
            Self::MovieOrVideoScreenCapture => String::from("MovieOrVideoScreenCapture"),
            Self::ABrightColouredFish => String::from("ABrightColouredFish"),
            Self::Illustration => String::from("Illustration"),
            Self::BandOrArtistLogotype => String::from("BandOrArtistLogotype"),
            Self::PublisherOrStudioLogoType => String::from("PublisherOrStudioLogoType"),
        }
    }
}
