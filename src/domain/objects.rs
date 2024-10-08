#[derive(Debug)]
pub struct RecentTrack {
    pub artist_name: String,
    pub title: String,
    pub lastfm_url: String,
    pub album_title: String,
    pub date: String,
    pub lastfm_image_url: Option<String>,
}

#[derive(Debug)]
pub struct Track {
    pub artist_name: String,
    pub title: String,
    pub lastfm_url: String,
    pub track_number: u64,
}

#[derive(Debug)]
pub struct AlbumInfo {
    pub artist_name: String,
    pub lastfm_image_url: Option<String>,
    pub title: String,
    pub tracks: Vec<Track>,
    pub lastfm_url: String,
}

#[derive(Debug)]
pub struct ScrobbleTrackPayload {
    pub track_title: String,
    pub artist_name: String,
    pub timestamp: u64,
    pub track_number: Option<u64>,
    pub album_title: Option<String>,
}

#[derive(Debug)]
pub struct TrackScrobblingResult {
    pub accepted: bool,
}
