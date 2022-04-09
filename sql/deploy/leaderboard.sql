-- Deploy emojied:leaderboard to pg

BEGIN;

CREATE FUNCTION app.leaderboard()
  RETURNS TABLE(identifier TEXT, clicks BIGINT, url TEXT)
  LANGUAGE sql
  AS $$
    SELECT links.identifier
         , links.clicks
         , concat(links.scheme, '://', hosts.name, links.path) AS url
      FROM app.links
      JOIN app.hosts
      ON links.host = hosts.host_id
      ORDER BY links.clicks DESC
      LIMIT 20;
  $$;

COMMIT;
