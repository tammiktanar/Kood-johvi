-- Lists following and users that have sent a message

WITH known AS (
    SELECT CASE WHEN sender != ?1 THEN sender ELSE receiver END knownID, max(messageID) as sortKey
    FROM messageUser
    WHERE sender = ?1
       OR receiver = ?1
    GROUP BY knownID
    ORDER BY sortKey DESC
)

SELECT u.*
FROM (SELECT u.*
      FROM known k
               JOIN user u on u.userID = k.knownID

      UNION

      SELECT u.*
      FROM follow f
               JOIN user u ON f.followingID = u.userID
      WHERE f.followerID = ?1) u
LEFT JOIN known k ON u.userID = k.knownID
WHERE userID != 0
ORDER BY k.sortKey DESC;
