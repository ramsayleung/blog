-- Your SQL goes here
CREATE TABLE category (
    id character varying(64) NOT NULL,
    name character varying(64) NOT NULL,
    description text,
    create_time date NOT NULL,
    modify_time date NOT NULL
)
