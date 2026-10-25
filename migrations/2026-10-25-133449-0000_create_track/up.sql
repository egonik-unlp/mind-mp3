-- Your SQL goes here
create table if not exists tracks (
  id serial primary key ,
  track_name varchar(255) not null,
  track_artist varchar(255 ) not null,
  thought_id serial not null,
  created_at date not null,
  FOREIGN KEY(thought_id) REFERENCES thoughts(id)
);
