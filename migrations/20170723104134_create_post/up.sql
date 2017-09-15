-- Your SQL goes here
CREATE TABLE post (
id serial PRIMARY KEY,
title text NOT NULL,
subtitle text NOT NULL,
raw_content text NOT NULL,
rendered_content text NOT NULL,
create_time timestamp NOT NULL,
modify_time timestamp NOT NULL,
post_type integer Not Null,
hit_time integer NOT NULL,
published BOOLEAN NOT NULL DEFAULT FALSE
)
