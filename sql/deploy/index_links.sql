-- Deploy emojied:index_links to pg

BEGIN;

CREATE INDEX identifier_index ON app.links (identifier);

COMMIT;
