#[derive(Debug)]
#[allow(dead_code)]
#[derive(Default)]
pub struct BlockCueSheet {
    pub media_catalog: String,
    pub lead_in_samples_number: u64,
    pub is_cd: bool,
    pub tracks_number: u8,
    pub tracks: Vec<Track>,
}

impl BlockCueSheet {
    pub fn new(
        media_catalog: String,
        lead_in_samples_number: u64,
        is_cd: bool,
        tracks_number: u8,
        tracks: Vec<Track>,
    ) -> Self {
        BlockCueSheet {
            media_catalog,
            lead_in_samples_number,
            is_cd,
            tracks_number,
            tracks,
        }
    }
}



#[derive(Debug)]
#[allow(dead_code)]
#[derive(Default)]
pub struct Track {
    offset: u64,
    number: u8,
    isrc: String,
    is_audio_track: bool,
    pre_emphasis: bool,
    track_index_points_number: u8,
    track_indices: Vec<TrackIndex>,
}



impl Track {
    pub fn new(
        offset: u64,
        number: u8,
        isrc: String,
        is_audio_track: bool,
        pre_emphasis: bool,
        track_index_points_number: u8,
        track_indices: Vec<TrackIndex>,
    ) -> Self {
        Track {
            offset,
            number,
            isrc,
            is_audio_track,
            pre_emphasis,
            track_index_points_number,
            track_indices,
        }
    }
}
#[derive(Debug)]
#[allow(dead_code)]
#[derive(Default)]
pub struct TrackIndex {
    offset: u64,
    index_point_number: u8,
}



impl TrackIndex {
    pub fn new(offset: u64, index_point_number: u8) -> Self {
        TrackIndex {
            offset,
            index_point_number,
        }
    }
}
