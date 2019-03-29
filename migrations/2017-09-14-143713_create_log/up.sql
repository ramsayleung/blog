-- Your SQL goes here
CREATE TABLE public.visitor_log (
id serial PRIMARY KEY,
ip inet NOT NULL,
access_time timestamp without time zone NOT NULL,
user_id integer DEFAULT 0 NOT NULL
);
