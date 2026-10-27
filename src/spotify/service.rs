use crate::spotify::{client::SpotifyClient, dto::SpotifyResultDto};

#[derive(Debug, Clone)]
pub struct SpotifySearchService {
    client: SpotifyClient,
}

impl SpotifySearchService {
    pub fn new(client: SpotifyClient) -> Self {
        Self { client }
    }
    pub fn search(&self, query: String) -> anyhow::Result<SpotifyResultDto> {
        self.client.search(query)
    }
}

