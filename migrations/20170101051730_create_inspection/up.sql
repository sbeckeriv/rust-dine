CREATE TABLE inspections (
  id SERIAL PRIMARY KEY,
  place_id INTEGER NOT NULL,
  title VARCHAR NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f',
  closed BOOLEAN NOT NULL DEFAULT 'f',
  inspected_at TIMESTAMP NOT NULL,
  inspection_type VARCHAR NOT NULL,
  inspection_score INTEGER NOT NULL
)
