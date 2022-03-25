-- Deploy emojiurl:extensions to pg

BEGIN;

CREATE EXTENSION IF NOT EXISTS pgtap;

COMMIT;
