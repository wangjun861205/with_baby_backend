-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS playings_uploads_playing_id;
DROP INDEX IF EXISTS eatings_uploads_eating_id;
DROP TABLE IF EXISTS eatings_uploads;