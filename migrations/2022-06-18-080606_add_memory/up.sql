CREATE TABLE IF NOT EXISTS memories (
    id SERIAL NOT NULL,
    title VARCHAR NOT NULL,
    content TEXT NOT NULL,
    owner INT NOT NULL REFERENCES users (id),
    location INT NOT NULL REFERENCES locations (id),
    create_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id)
);

CREATE TRIGGER update_memories_update_on BEFORE UPDATE ON memories FOR EACH ROW EXECUTE PROCEDURE update_update_on();

CREATE TABLE IF NOT EXISTS memory_upload_rels (
    id SERIAL NOT NULL,
    memory INT NOT NULL REFERENCES memories (id),
    upload INT NOT NULL REFERENCES uploads (id),
    PRIMARY KEY (id)
);