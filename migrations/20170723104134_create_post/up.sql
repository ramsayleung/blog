-- Your SQL goes here
CREATE TABLE post (
id character varying(64) NOT NULL PRIMARY KEY,
title text NOT NULL,
subtitle text NOT NULL,
create_time timestamp NOT NULL,
publish_time timestamp NOT NULL,
modify_time timestamp NOT NULL,
published BOOLEAN NOT NULL DEFAULT FALSE,
user_id character varying(64) NOT NULL
)
