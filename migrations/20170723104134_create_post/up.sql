CREATE TABLE post (
id serial PRIMARY KEY,
title varchar NOT NULL,
subtitle varchar NOT NULL,
raw_content text NOT NULL,
rendered_content text NOT NULL,
create_time timestamp without time zone NOT NULL,
modify_time timestamp without time zone NOT NULL,
post_type integer NOT NULL,
hit_time integer NOT NULL,
published boolean DEFAULT false NOT NULL,
slug_url varchar NOT NULL
);
