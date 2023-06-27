SELECT *
FROM messageUser
WHERE (?3 = 0 OR messageID < ?3)
  AND ((sender = ?1 AND receiver = ?2)
    OR (sender = ?2 AND receiver = ?1))
ORDER BY messageID DESC
LIMIT 10;
