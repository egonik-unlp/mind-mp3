use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotifyResultDto {
    track_artist: String,
    track_name: String,
}

impl SpotifyResultDto {
    pub fn new(track_artist: String, track_name: String) -> Self {
        Self {
            track_artist,
            track_name,
        }
    }
}
