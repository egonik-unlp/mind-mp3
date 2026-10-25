use crate::{
    core::database::{connection::DbPool, vector_store::VectorDbPool},
    thought::repository::ThougtRepository,
    thought::service::ThoughtService,
};

#[derive(Debug, Clone)]
pub struct AppState {
    thought_service: ThoughtService,
}

impl AppState {
    pub fn new(pool: DbPool) -> Self {
        let repo = ThougtRepository::new(pool);
        let service = ThoughtService::new(repo);
        Self {
            thought_service: service,
        }
    }
}
