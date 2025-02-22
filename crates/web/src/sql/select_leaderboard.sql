SELECT
  l.id,
  l.short_name,
  l.clicks,
  l.target_link
FROM links l
ORDER BY
  l.clicks
LIMIT 20
