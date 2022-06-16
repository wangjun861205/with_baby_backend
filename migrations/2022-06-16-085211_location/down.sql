DROP TRIGGER IF EXISTS update_comments_udpate_on ON comments;
DROP TABLE IF EXISTS comments;

DROP TRIGGER IF EXISTS update_equipments_update_on ON equipments;
DROP TABLE IF EXISTS equipments; 

DROP INDEX IF EXISTS locations_latitude_longitude;
DROP TRIGGER IF EXISTS update_locations_update_on ON locations;
DROP TABLE IF EXISTS locations;