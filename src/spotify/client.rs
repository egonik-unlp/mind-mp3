use anyhow::Ok;
use reqwest::{Client, ClientBuilder};

use crate::spotify::dto::SpotifyResultDto;

#[derive(Debug, Clone)]
pub struct SpotifyClient {
    http: Client,
    client_id: String,
    client_secret: String,
    endpoint: String,
}
impl SpotifyClient {
    pub fn new(client_id: String, client_secret: String, endpoint: String) -> Self {
        let http = Client::new();
        Self {
            http,
            client_id,
            client_secret,
            endpoint,
        }
    }
    fn _search_inner(query: String) -> anyhow::Result<SpotifyResultDto> {
        todo!()
    }
    pub fn search(&self, query: String) -> anyhow::Result<SpotifyResultDto> {
        Ok(SpotifyResultDto::new(
            "DJ Koze".into(),
            "Wennpennest".into(),
        ))
    }
}
