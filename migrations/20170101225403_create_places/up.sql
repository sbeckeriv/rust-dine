CREATE TABLE places (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  program_identifier VARCHAR NOT NULL,
  description VARCHAR,
  phone VARCHAR,
  address VARCHAR NOT NULL,
  longitude double precision NOT NULL,
  latitude double precision NOT NULL
);
CREATE UNIQUE INDEX places_create_idx ON places(name,address);
CREATE INDEX places_lat_idx ON places(latitude);
CREATE INDEX places_long_idx ON places(longitude);
CREATE INDEX places_lat_long_idx ON places(latitude, longitude);
