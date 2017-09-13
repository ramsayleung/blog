-- Your SQL goes here
CREATE TABLE "user" (
id serial PRIMARY KEY,
username character varying(64) NOT NULL,
hashed_password character varying(128) NOT NULL,
create_time timestamp NOT NULL,
modify_time timestamp NOT NULL,
email character varying(128) UNIQUE NOT NULL,
avatar_url character varying(128)
)
