use std::{
    ops::Add,
    time::{Duration, Instant},
};

use anyhow::{Context, Ok};
use reqwest::Client;
use serde::Deserialize;

use crate::spotify::dto::SpotifyResultDto;

const SPOTIFY_TOKEN_URL: &'static str = "https://accounts.spotify.com/api/token";

#[derive(Debug, Clone)]
pub struct SpotifyClient {
    http: Client,
    client_id: String,
    client_secret: String,
    endpoint: String,
    token: Option<Token>,
}

#[derive(Debug, Deserialize)]
struct SpotifyResponse {
    access_token: String,
    expires_in: u64,
}

#[derive(Debug, Clone)]
struct Token {
    pub expiration_time: Instant,
    pub token: String,
}

impl SpotifyClient {
    pub fn new(client_id: String, client_secret: String, endpoint: String) -> Self {
        let http = Client::new();
        let token = None;
        Self {
            http,
            client_id,
            client_secret,
            endpoint,
            // TODO: Esto tiene que tener un mutex
            token,
        }
    }

    /// TODO: El ownership acá está bastante para el ojete
    async fn token(&mut self) -> anyhow::Result<Token> {
        if self
            .token
            .as_ref()
            .is_some_and(|token| token.expiration_time.gt(&Instant::now()))
        {
            return Ok(self.clone().token.unwrap());
        }

        let response: SpotifyResponse = self
            .http
            .post(SPOTIFY_TOKEN_URL)
            .basic_auth(self.client_id.clone(), Some(self.client_secret.clone()))
            .send()
            .await
            .context("Getting token")?
            .json()
            .await?;
        let expiration_time = Instant::now().add(Duration::from_secs(response.expires_in));
        let token = Token {
            expiration_time,
            token: response.access_token,
        };
        self.token = Some(token.clone());
        Ok(token)
    }

    fn _search_inner(query: String) -> anyhow::Result<SpotifyResultDto> {
        todo!()
    }
    pub async fn search(&mut self, query: String) -> anyhow::Result<SpotifyResultDto> {
        let token = self.token().await.context("Getting token")?;
        self.http
            .get(self.endpoint.clone())
            .bearer_auth(token.token)
            .send()
            .await
            .context("Sending request to spotify")?
            .json()
            .await
            .map_err(Into::into)
    }
}
