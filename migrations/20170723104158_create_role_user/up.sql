-- Your SQL goes here
CREATE TABLE user_role (
    id character varying(64) NOT NULL,
    user_id character varying(64) NOT NULL,
    role_id character varying(64) NOT NULL,
    create_time date NOT NULL,
    modify_time date NOT NULL
)
