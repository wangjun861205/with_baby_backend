-- Your SQL goes here
CREATE TABLE locations (
	id SERIAL NOT NULL,
	name VARCHAR NOT NULL,
	latitude DOUBLE PRECISION NOT NULL,
	longitude DOUBLE PRECISION NOT NULL,
	category INT NOT NULL,
	description TEXT NOT NULL,
	discoverer INT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
	create_on TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
	update_on TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
	PRIMARY KEY (id)
);

CREATE INDEX locations_latitude_longitude ON locations USING gist (ll_to_earth(latitude, longitude));

CREATE TRIGGER update_locations_update_on
	BEFORE UPDATE
	ON
		locations
	FOR EACH ROW
	EXECUTE PROCEDURE update_update_on();



CREATE TABLE equipments (
	id SERIAL NOT NULL,
	name VARCHAR NOT NULL,
	is_required BOOLEAN NOT NULL,
	usage TEXT NOT NULL,
	location INT NOT NULL REFERENCES locations (id) ON DELETE CASCADE,
	create_on TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
	update_on TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
	PRIMARY KEY (id)
);

CREATE TRIGGER update_equipments_update_on
	BEFORE UPDATE
	ON
		equipments
	FOR EACH ROW
	EXECUTE PROCEDURE update_update_on();

CREATE TABLE comments (
	id SERIAL NOT NULL,
	rank INT NOT NULL,
	content TEXT NOT NULL,
	"user" INT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
	location INT NOT NULL REFERENCES locations (id) ON DELETE CASCADE,
	create_on TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
	update_on TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
	PRIMARY KEY (id)
);

CREATE TRIGGER update_comments_update_on
	BEFORE UPDATE
	ON
		equipments
	FOR EACH ROW
	EXECUTE PROCEDURE update_update_on();