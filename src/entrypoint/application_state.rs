use axum::extract::FromRef;
use leptos::config::LeptosOptions;

use crate::{
    core::database::connection::DbPool,
    // vector_store::VectorDbPool},
    thought::repository::ThougtRepository,
    thought::service::ThoughtService,
};

#[derive(Debug, Clone, FromRef)]
pub struct AppState {
    pub thought_service: ThoughtService,
    pub leptos_options: LeptosOptions,
}

impl AppState {
    pub fn new(pool: DbPool, leptos_options: LeptosOptions) -> Self {
        let repo = ThougtRepository::new(pool);
        let service = ThoughtService::new(repo);
        Self {
            thought_service: service,
            leptos_options,
        }
    }
}
