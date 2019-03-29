CREATE TABLE post (
id integer NOT NULL,
title character varying NOT NULL,
subtitle character varying NOT NULL,
raw_content text NOT NULL,
rendered_content text NOT NULL,
create_time timestamp without time zone NOT NULL,
modify_time timestamp without time zone NOT NULL,
post_type integer NOT NULL,
hit_time integer NOT NULL,
published boolean DEFAULT false NOT NULL,
slug_url character varying NOT NULL,
enable_comment boolean DEFAULT true NOT NULL
);
