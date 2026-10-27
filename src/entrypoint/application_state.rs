use axum::extract::FromRef;
use leptos::config::LeptosOptions;

use crate::{
    core::database::connection::DbPool,
    spotify::{client::SpotifyClient, service::SpotifySearchService},
    thought::{repository::ThougtRepository, service::ThoughtService},
};

#[derive(Debug, Clone, FromRef)]
pub struct AppState {
    pub thought_service: ThoughtService,
    pub spotify_service: SpotifySearchService,
    pub leptos_options: LeptosOptions,
}

impl AppState {
    pub fn new(pool: DbPool, spotify_client: SpotifyClient, leptos_options: LeptosOptions) -> Self {
        let thought_repo = ThougtRepository::new(pool);
        let thought_service = ThoughtService::new(thought_repo);
        let spotify_service = SpotifySearchService::new(spotify_client);
        Self {
            thought_service,
            spotify_service,
            leptos_options,
        }
    }
}
