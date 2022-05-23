-- This file should undo anything in `up.sql`
drop trigger if exists update_playings on playings cascade;
drop index playings_latitude_longitude;
drop table playings;
drop trigger if exists update_eatings on eatings cascade;
drop index eatings_latitude_longitude;
drop table eatings;
drop trigger if exists udpate_users on users cascade;
drop table users;
drop function update_update_on;
