SELECT p.*, u.*
FROM post p
         JOIN user u on u.userID = p.authorID
WHERE (?2 < 1 OR p.postID < ?2)
  AND groupID = ?1
ORDER BY p.postID DESC
LIMIT 20;
