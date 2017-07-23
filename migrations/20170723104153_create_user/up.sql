-- Your SQL goes here
CREATE TABLE "user" (
    id character varying(64) NOT NULL,
    username character varying(64) NOT NULL,
    password character varying(128) NOT NULL,
    create_time date NOT NULL,
    modify_time date NOT NULL,
    salt character varying(64) NOT NULL,
    email character varying(128) NOT NULL,
    avatar_url character varying(128)
)
