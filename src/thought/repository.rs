use crate::core::database::connection::interact_with_connection;
use crate::core::database::connection::DbPool;
use crate::core::database::schema::thoughts;
use crate::thought::dto::ThoughtDto;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use std::future::Future;

#[derive(Debug, Clone)]
pub struct ThougtRepository {
    pool: DbPool,
}

impl ThougtRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
    pub async fn create_thought(&self, input: ThoughtDto) -> anyhow::Result<()> {
        interact_with_connection(&self.pool, |conn| insert_into(thoughts))
    }
}
