CREATE TABLE inspections (
  id SERIAL PRIMARY KEY,
  place_id INTEGER NOT NULL,
  title VARCHAR NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f',
  closed BOOLEAN NOT NULL DEFAULT 'f',
  inspected_at TIMESTAMP NOT NULL,
  inspection_type VARCHAR NOT NULL,
  inspection_score INTEGER NOT NULL
);
CREATE UNIQUE INDEX inspections_create_idx ON inspections(place_id,inspected_at);
CREATE INDEX inspections_place_id_idx ON inspections(place_id);
