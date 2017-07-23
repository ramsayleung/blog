-- Your SQL goes here
CREATE TABLE post (
    id character varying(64) NOT NULL,
    title text NOT NULL,
    subtitle text NOT NULL,
    create_time date NOT NULL,
    publish_time date NOT NULL,
    modify_time date NOT NULL,
    status character varying(20) NOT NULL,
    user_id character varying(64) NOT NULL
)
