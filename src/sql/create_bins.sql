CREATE TABLE IF NOT EXISTS bins (
  uuid TEXT NOT NULL UNIQUE,
  data TEXT NOT NULL,
  name TEXT NOT NULL,
  hidden INTEGER NOT NULL, -- 1 hidden; 0 public
  time INTEGER NOT NULL
);
