CREATE TABLE violations (
  id SERIAL PRIMARY KEY,
  inspection_id INTEGER NOT NULL,
  kind VARCHAR NOT NULL,
  points INTEGER NOT NULL,
  description VARCHAR NOT NULL
);
CREATE UNIQUE INDEX violations_create_idx ON violations(inspection_id,kind,points,description);
CREATE INDEX violations_inspection_id_idx ON violations(inspection_id);

