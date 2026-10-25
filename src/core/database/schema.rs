// @generated automatically by Diesel CLI.

diesel::table! {
    thoughts (id) {
        id -> Int4,
        created_at -> Date,
    }
}

diesel::table! {
    tracks (id) {
        id -> Int4,
        #[max_length = 255]
        track_name -> Varchar,
        #[max_length = 255]
        track_artist -> Varchar,
        thought_id -> Int4,
        created_at -> Date,
    }
}

diesel::joinable!(tracks -> thoughts (thought_id));

diesel::allow_tables_to_appear_in_same_query!(thoughts, tracks,);
