-- This file should undo anything in `up.sql`
drop trigger update_playings on playings cascade;
drop index playings_latitude_longitude;
drop table playings;
drop trigger update_eatings on eatings cascade;
drop index eatings_latitude_longitude;
drop table eatings;
drop function update_update_on;
