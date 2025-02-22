-- Deploy emojied:20250215161708_add_links to sqlite

BEGIN;

  CREATE TABLE links (
    id INTEGER PRIMARY KEY,
    short_name TEXT NOT NULL CHECK (length(short_name) > 0 AND length(short_name) < 40),
    target_link TEXT NOT NULL,
    clicks INTEGER NOT NULL DEFAULT 0
  ) STRICT;

  CREATE UNIQUE INDEX idx_links_short_name ON links(short_name);

COMMIT;
