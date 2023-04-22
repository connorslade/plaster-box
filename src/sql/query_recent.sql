SELECT uuid,
    name,
    time
FROM bins
WHERE hidden = 0
ORDER BY time DESC
LIMIT ? OFFSET ?