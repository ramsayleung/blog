-- Your SQL goes here
CREATE TABLE "user" (
id serial PRIMARY KEY,
username character varying(64) NOT NULL,
hashed_password character varying(128) NOT NULL,
create_time timestamp NOT NULL,
modify_time timestamp NOT NULL,
email character varying(128) UNIQUE NOT NULL,
avatar_url character varying(128)
);

INSERT INTO "user" VALUES (1,'admin','$2y$10$QFkjuKUBF3s1ldzPFB8/WejgRJ9nW2CdXOSxfQJdnYXDcoIzrBkzS',current_timestamp,current_timestamp,'admin','https://imgur.com/a/N6Y97')
