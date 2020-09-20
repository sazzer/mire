DROP TABLE abilities;

CREATE TABLE stats(
  stat_id UUID PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  description TEXT NOT NULL,
  default_value INTEGER NOT NULL CHECK (default_value >= 0)
);