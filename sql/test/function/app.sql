BEGIN;

SELECT plan(6);

-- Fresh start
DELETE FROM app.links;
DELETE FROM app.domains;

--------------------------------------------------------------------------------

-- #1
-- Check if relevant insert_url exists
SELECT has_function('app', 'insert_url', 'Check if `insert_url` exists');

-- #2
-- Check if relevant get_url exists
SELECT has_function('app', 'get_url', 'Check if `get_url` exists');

-- #3
-- If the domain doesn't exist then it should insert it in the database.
SELECT row_eq(
  $$ SELECT * FROM app.insert_url('🍊🌐', 'https'::app.HTTP_PROTOCOL, 'news', 'ycombinator.com', '/') $$,
  ROW('🍊🌐'::TEXT));

-- #4
SELECT row_eq($$ SELECT name FROM app.domains $$, ROW('ycombinator.com'::TEXT));

-- #5
-- Should be able to insert an entry of an existing domain
SELECT row_eq(
  $$ SELECT * FROM app.insert_url('🍊', 'https'::app.HTTP_PROTOCOL, 'news', 'ycombinator.com', '/item?id=1') $$,
  ROW('🍊'::TEXT));

-- #6
-- Inserting an entry with the same identifier (emojis) is not allowed.
PREPARE insert_duplicate_url AS (
  SELECT *
    FROM app.insert_url(
      '🍊🌐',
      'https'::app.HTTP_PROTOCOL,
      'news',
      'ycombinator.com',
      '/item?id=30808944'
    )
);

SELECT throws_ok(
  'insert_duplicate_url',
  '23505',
  'duplicate key value violates unique constraint "links_identifier_key"'
);

-- TODO: Test if the values are in the proper columns

--------------------------------------------------------------------------------

SELECT * FROM finish();

ROLLBACK;
