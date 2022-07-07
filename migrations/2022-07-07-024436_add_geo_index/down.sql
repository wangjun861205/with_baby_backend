ALTER TABLE locations DROP CONSTRAINT uni_geo_index;
ALTER TABLE locations DROP COLUMN geo_index;
ALTER TABLE location_upload_rels 
DROP CONSTRAINT location_upload_rels_location_id_fkey, 
DROP CONSTRAINT location_upload_rels_upload_id_fkey, 
ADD CONSTRAINT location_upload_rels_location_id_fkey FOREIGN KEY (location_id) REFERENCES locations (id), 
ADD CONSTRAINT location_upload_rels_upload_id_fkey FOREIGN KEY (upload_id) REFERENCES uploads (id);