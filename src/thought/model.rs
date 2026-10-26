use chrono::Utc;
use diesel::prelude::*;
use diesel::{deserialize::Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::thought::dto::{ThoughtDto, TrackDto};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name= crate::core::database::schema::thoughts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Thought {
    id: i32,
    created_at: chrono::NaiveDate,
    context: String,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations,
)]
#[diesel(belongs_to(Thought))]
#[diesel(table_name= crate::core::database::schema::tracks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Track {
    id: i32,
    track_artist: String,
    track_name: String,
    thought_id: i32,
    created_at: chrono::NaiveDate,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = crate::core::database::schema::thoughts)]
pub struct InsertableThought {
    pub created_at: chrono::NaiveDate,
    pub context: String,
}
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = crate::core::database::schema::tracks)]
pub struct InsertableTrack {
    track_artist: String,
    track_name: String,
    created_at: chrono::NaiveDate,
    thought_id: i32,
}

impl TrackDto {
    pub fn to_insertable(self, thought_id: i32) -> InsertableTrack {
        let TrackDto {
            track_artist,
            track_name,
        } = self;
        let created_at = Utc::now().date_naive();
        InsertableTrack {
            created_at,
            track_artist,
            track_name,
            thought_id,
        }
    }
}
impl From<Track> for TrackDto {
    fn from(value: Track) -> Self {
        let Track {
            id,
            track_artist,
            track_name,
            thought_id,
            created_at,
        } = value;

        Self {
            track_artist,
            track_name,
        }
    }
}

impl From<(Vec<Track>, Thought)> for ThoughtDto {
    fn from(value: (Vec<Track>, Thought)) -> Self {
        let Thought {
            id,
            created_at,
            context,
        } = value.1;
        let track = value.0.first().unwrap().clone().into();
        ThoughtDto { track, context }
    }
}

impl From<ThoughtDto> for InsertableThought {
    fn from(value: ThoughtDto) -> Self {
        let created_at = Utc::now().date_naive();
        let context = value.context;
        Self {
            created_at,
            context,
        }
    }
}
