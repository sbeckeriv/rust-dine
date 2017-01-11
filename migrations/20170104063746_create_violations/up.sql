CREATE TABLE violations (
  id SERIAL PRIMARY KEY,
  inspection_id INTEGER NOT NULL,
  kind VARCHAR NOT NULL,
  points INTEGER NOT NULL,
  description VARCHAR NOT NULL
)

