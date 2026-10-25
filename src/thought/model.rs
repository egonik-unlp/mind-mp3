use diesel::prelude::*;
use diesel::{deserialize::Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::thought::dto::ThoughtDto;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name= crate::core::database::schema::thoughts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Thought {
    id: i32,
    created_at: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(Thought))]
#[diesel(table_name= crate::core::database::schema::tracks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Track {
    id: i32,
    track_artist: String,
    track_name: String,
    thought_id: i32,
    created_at: chrono::NaiveDate,
}
