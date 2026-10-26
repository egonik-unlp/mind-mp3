use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThoughtDto {
    pub track: TrackDto,
    pub context: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrackDto {
    pub track_artist: String,
    pub track_name: String,
}
