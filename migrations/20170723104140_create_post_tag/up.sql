-- Your SQL goes here
CREATE TABLE post_tag (
    id character varying(64) NOT NULL,
    post_id character varying(64) NOT NULL,
    tag_id character varying(64) NOT NULL,
    modify_time date,
    create_time date NOT NULL
)
