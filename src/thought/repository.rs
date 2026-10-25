use crate::core::database::connection::interact_with_connection;
use crate::core::database::connection::DbPool;
use crate::core::database::schema::thoughts;
use crate::core::database::schema::tracks;
use crate::thought::dto::ThoughtDto;
use crate::thought::dto::TrackDto;
use crate::thought::model::InsertableThought;
use crate::thought::model::Thought;
use crate::thought::model::Track;
use diesel::dsl::insert_into;
use diesel::prelude::*;

#[derive(Debug, Clone)]
pub struct ThougtRepository {
    pool: DbPool,
}

impl ThougtRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
    pub async fn create_thought(&self, input: ThoughtDto) -> anyhow::Result<()> {
        let insertable_thought: InsertableThought = input.clone().into();
        interact_with_connection(&self.pool, |conn| {
            let thought_id = insert_into(thoughts::table)
                .values(insertable_thought)
                .returning(thoughts::id)
                .get_result::<i32>(conn)?;
            let insertable_track = input.track.to_insertable(thought_id);
            insert_into(tracks::table)
                .values(insertable_track)
                .execute(conn)
                .map(|_| ())
        })
        .await
    }
    pub async fn get_all_thoughts(&self) -> anyhow::Result<Vec<ThoughtDto>> {
        interact_with_connection(&self.pool, |conn| {
            thoughts::table
                .select(Thought::as_select())
                .load(conn)
                .and_then(|thoughts| {
                    Track::belonging_to(&thoughts)
                        .select(Track::as_select())
                        .load(conn)
                        .map(|tracks| {
                            tracks
                                .grouped_by(&thoughts)
                                .into_iter()
                                .zip(thoughts)
                                // .map(Thought)
                                .map(ThoughtDto::from)
                                .collect()
                        })
                })
                .map_err(Into::into)
        })
        .await
    }
}
