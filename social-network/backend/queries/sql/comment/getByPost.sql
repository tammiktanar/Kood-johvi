SELECT c.*, u.*
FROM comment c
    JOIN user u on u.userID = c.authorID
WHERE postID = ?
--ORDER BY commentID DESC
;
