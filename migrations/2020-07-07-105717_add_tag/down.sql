-- This file should undo anything in `up.sql`
alter table post
drop column tag;
drop index if exists idx_tag;
