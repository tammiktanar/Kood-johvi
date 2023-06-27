SELECT p.*, u.*, g.*
FROM groupMember gM
         JOIN post p ON p.groupID = gM.groupID
         JOIN user u ON u.userID = p.authorID
         JOIN "group" g on gM.groupID = g.groupID
WHERE (?2 < 1 OR p.postID < ?2)
  AND gM.userID = ?1
ORDER BY p.postID DESC
LIMIT 20;
