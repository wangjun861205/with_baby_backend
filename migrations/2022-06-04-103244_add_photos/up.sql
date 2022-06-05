-- Your SQL goes here
CREATE TABLE playings_uploads (
    id SERIAL NOT NULL,
    playing_id INTEGER NOT NULL REFERENCES playings (id) ON DELETE CASCADE,
    upload_id INTEGER NOT NULL REFERENCES uploads (id) ON DELETE CASCADE,
    PRIMARY KEY (id),
    CONSTRAINT unq_playing_id_upload_id UNIQUE (playing_id, upload_id)
);
