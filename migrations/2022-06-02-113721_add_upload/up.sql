-- Your SQL goes here
CREATE TABLE uploads (
    id SERIAL NOT NULL,
    fetch_code VARCHAR NOT NULL,
    owner INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    create_on TIMESTAMP NOT NULL DEFAULT current_timestamp,
    update_on TIMESTAMP NOT NULL DEFAULT current_timestamp,
    PRIMARY KEY (id)
);

CREATE TRIGGER uploads_update_on BEFORE UPDATE ON uploads FOR EACH ROW EXECUTE PROCEDURE update_update_on();

