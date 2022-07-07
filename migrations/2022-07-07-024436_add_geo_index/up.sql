ALTER TABLE locations ADD COLUMN geo_index CHAR(16) NOT NULL;
ALTER TABLE locations ADD CONSTRAINT uni_geo_index UNIQUE (geo_index);
ALTER TABLE location_upload_rels 
DROP CONSTRAINT location_upload_rels_location_id_fkey, 
DROP CONSTRAINT location_upload_rels_upload_id_fkey, 
ADD CONSTRAINT location_upload_rels_location_id_fkey FOREIGN KEY (location_id) REFERENCES locations (id) ON DELETE CASCADE, 
ADD CONSTRAINT location_upload_rels_upload_id_fkey FOREIGN KEY (upload_id) REFERENCES uploads (id) ON DELETE CASCADE;
