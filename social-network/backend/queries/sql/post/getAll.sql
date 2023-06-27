WITH following AS (SELECT followingID
                   FROM follow f
                   WHERE f.followerID = ?1)

SELECT p.*, u.*
FROM post p
         JOIN user u on u.userID = p.authorID
WHERE (?2 < 1 OR p.postID < ?2)
  AND groupID IS NULL
  AND ( -- Filter out posts based on permission
            p.authorID = ?1 OR -- Is author
            (p.privacy = 'public') OR -- AND (u.private = FALSE OR p.authorID IN following)) OR
            (p.privacy = 'private' AND p.authorID IN following) OR
            (p.privacy = 'manual' AND EXISTS(SELECT * FROM postAllowedUser WHERE postID = p.postID AND userID = ?1))
    )
ORDER BY p.postID DESC
LIMIT 20;
