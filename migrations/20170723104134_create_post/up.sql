-- Your SQL goes here
-- CREATE TYPE post_type AS ENUM ('about', 'post');
CREATE TABLE post (
id serial PRIMARY KEY,
title text NOT NULL,
subtitle text NOT NULL,
raw_content text NOT NULL,
rendered_content text NOT NULL,
create_time timestamp NOT NULL,
modify_time timestamp NOT NULL,
-- post_type post_type Not Null,
published BOOLEAN NOT NULL DEFAULT FALSE
)
