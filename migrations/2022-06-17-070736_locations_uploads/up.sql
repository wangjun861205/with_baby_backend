-- Your SQL goes here
CREATE TABLE IF NOT EXISTS location_upload_rels (
	id SERIAL NOT NULL,
	location_id INT NOT NULL REFERENCES locations (id),
	upload_id INT NOT NULL REFERENCES uploads (id),
	PRIMARY KEY (id),
	CONSTRAINT unq_location_id_upload_id UNIQUE (location_id, upload_id)
);

CREATE INDEX IF NOT EXISTS location_upload_rel_location_id ON location_upload_rels USING BTREE (location_id);