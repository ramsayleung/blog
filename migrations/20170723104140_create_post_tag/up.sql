-- Your SQL goes here
CREATE TABLE post_tag (
    id character varying(64) NOT NULL PRIMARY KEY,
    post_id character varying(64) NOT NULL,
    tag_id character varying(64) NOT NULL,
    modify_time timestamp,
    create_time timestamp NOT NULL
)
