-- Your SQL goes here
CREATE TABLE IF NOT EXISTS eatings_uploads (
	id SERIAL NOT NULL,
	eating_id INTEGER NOT NULL REFERENCES eatings(id) ON DELETE CASCADE,
	upload_id INTEGER NOT NULL REFERENCES uploads(id) ON DELETE CASCADE,
	PRIMARY KEY (id)
);

CREATE INDEX IF NOT EXISTS eatings_uploads_eating_id ON eatings_uploads (eating_id);

CREATE INDEX IF NOT EXISTS playings_uploads_playing_id ON playings_uploads (playing_id);