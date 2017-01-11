CREATE TABLE places (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  program_identifier VARCHAR NOT NULL,
  description VARCHAR,
  phone VARCHAR,
  address VARCHAR NOT NULL,
  longitude double precision NOT NULL,
  latitude double precision NOT NULL
)
