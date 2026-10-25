use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ThoughtDto {
    song: Song,
}

#[derive(Debug, Serialize, Deserialize)]
struct Song {
    artist: String,
    track_name: String,
}
