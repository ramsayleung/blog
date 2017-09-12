-- Your SQL goes here
CREATE TABLE "user" (
id serial PRIMARY KEY,
username character varying(64) NOT NULL,
hashed_password integer[] NOT NULL,
create_time date NOT NULL,
modify_time date NOT NULL,
salt integer[] NOT NULL,
email character varying(128) NOT NULL,
avatar_url character varying(128)
)
