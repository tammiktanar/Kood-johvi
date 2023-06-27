SELECT mg.*, u.userID, u.firstname, u.lastname, u.nickname, u.image
FROM messageGroup mg
    JOIN user u on u.userID = mg.sender
WHERE (?3 = 0 OR messageID < ?3)
  AND groupID = ?2
ORDER BY messageID DESC
LIMIT 10;
