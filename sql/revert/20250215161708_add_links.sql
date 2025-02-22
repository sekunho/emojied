-- Revert emojied:20250215161708_add_links from sqlite

BEGIN;

  DROP TABLE links;

COMMIT;
