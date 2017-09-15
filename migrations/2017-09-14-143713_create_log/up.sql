-- Your SQL goes here
CREATE TABLE visitor_log (
id serial PRIMARY KEY,
ip inet NOT NULL,
access_time timestamp NOT NULL,
user_id integer NOT NULL DEFAULT 0
)
