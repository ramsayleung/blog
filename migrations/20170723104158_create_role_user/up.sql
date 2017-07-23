-- Your SQL goes here
CREATE TABLE user_role (
id character varying(64) NOT NULL PRIMARY KEY,
user_id character varying(64) NOT NULL,
role_id character varying(64) NOT NULL,
create_time timestamp NOT NULL,
modify_time timestamp NOT NULL
)
