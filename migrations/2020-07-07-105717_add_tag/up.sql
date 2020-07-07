-- Your SQL goes here
alter table post
add column tag jsonb NOT NULL;
create index idx_tag on post(tag);
