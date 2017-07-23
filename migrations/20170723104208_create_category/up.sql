-- Your SQL goes here
CREATE TABLE category (
id character varying(64) NOT NULL PRIMARY KEY,
name character varying(64) NOT NULL,
description text,
create_time timestamp NOT NULL,
modify_time timestamp NOT NULL
)
