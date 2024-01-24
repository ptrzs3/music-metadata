#[derive(Debug)]
#[allow(dead_code)]
pub struct BlockCueSheet {
    media_catalog: String,
    lead_in_samples_number: u64,
    is_cd: bool,
    tracks_number: u8,
    tracks: Vec<Track>,
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

impl Default for BlockCueSheet {
    fn default() -> Self {
        BlockCueSheet {
            media_catalog: String::default(),
            lead_in_samples_number: u64::default(),
            is_cd: bool::default(),
            tracks_number: u8::default(),
            tracks: Vec::default(),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Track {
    offset: u64,
    number: u8,
    isrc: String,
    is_audio_track: bool,
    pre_emphasis: bool,
    track_index_points_number: u8,
    track_indices: Vec<TrackIndex>,
}

impl Default for Track {
    fn default() -> Self {
        Track {
            offset: u64::default(),
            number: u8::default(),
            isrc: String::default(),
            is_audio_track: bool::default(),
            pre_emphasis: bool::default(),
            track_index_points_number: u8::default(),
            track_indices: Vec::default(),
        }
    }
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
pub struct TrackIndex {
    offset: u64,
    index_point_number: u8,
}

impl Default for TrackIndex {
    fn default() -> Self {
        TrackIndex {
            offset: u64::default(),
            index_point_number: u8::default(),
        }
    }
}

impl TrackIndex {
    pub fn new(offset: u64, index_point_number: u8) -> Self {
        TrackIndex {
            offset,
            index_point_number,
        }
    }
}
