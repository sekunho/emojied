-- Deploy emojied:links to pg
-- requires: extensions

BEGIN;

--------------------------------------------------------------------------------

CREATE SCHEMA app;

CREATE TYPE app.SCHEME AS ENUM ('http', 'https');

--------------------------------------------------------------------------------

CREATE TABLE app.hosts (
  host_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT NOT NULL UNIQUE
);

COMMENT ON TABLE app.hosts IS 'List of hosts';

CREATE TABLE app.links (
  link_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  identifier TEXT NOT NULL,
  scheme app.SCHEME NOT NULL,
  host UUID REFERENCES app.hosts(host_id),
  path TEXT NOT NULL,
  clicks BIGINT DEFAULT 0,

  UNIQUE(identifier)
);

COMMENT ON TABLE app.links IS 'Contains the links and their information';

--------------------------------------------------------------------------------

CREATE FUNCTION app.get_url(query TEXT)
  RETURNS TEXT
  LANGUAGE sql
  AS $$
    -- Considered as a "clicked" link whenever this gets triggered
    UPDATE app.links
      SET clicks = clicks + 1
      WHERE links.identifier = $1;

    SELECT concat(scheme, '://', hosts.name, path) AS url
      FROM app.links
      JOIN app.hosts
      ON links.host = hosts.host_id
      WHERE links.identifier = $1;
  $$;

CREATE FUNCTION app.insert_url(
    identifier TEXT, --CHECK length(identifier) > 0,
    scheme     TEXT,
    host       TEXT,
    path       TEXT -- CHECK length(identifier) > 0
  )
  RETURNS TEXT
  LANGUAGE sql
  AS $$
    INSERT
      INTO app.hosts (name)
      VALUES ($3)
      ON CONFLICT (name) DO NOTHING;

    WITH host_cte AS (
      SELECT host_id
        FROM app.hosts
        WHERE hosts.name = $3
    )
    INSERT INTO app.links (identifier, scheme, host, path)
      (
        SELECT
          $1 AS identifier,
          $2::app.SCHEME AS scheme,
          host_cte.host_id AS host,
          $4 AS path
        FROM host_cte
      )
      RETURNING identifier;
  $$;

CREATE FUNCTION app.get_url_stats(identifier TEXT)
  RETURNS TABLE(identifier TEXT, clicks BIGINT, url TEXT)
  LANGUAGE sql
  AS $$
    SELECT links.identifier
         , links.clicks
         , concat(links.scheme, '://', hosts.name, links.path) AS url
      FROM app.links
      JOIN app.hosts
      ON host_id = hosts.host_id
      WHERE links.identifier = $1;
  $$;

--------------------------------------------------------------------------------

COMMIT;
