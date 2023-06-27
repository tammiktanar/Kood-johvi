WITH following AS (SELECT followingID
                   FROM follow f
                   WHERE f.followerID = ?1)

SELECT p.*, u.*
FROM post p
         JOIN user u ON u.userID = p.authorID
WHERE (?2 < 1 OR p.postID < ?2)
  AND groupID IS NULL
  AND p.authorID IN following
  AND ( -- Filter out posts based on permission
            p.privacy = 'public' OR
            p.privacy = 'private' OR
            (p.privacy = 'manual' AND EXISTS(SELECT * FROM postAllowedUser WHERE postID = p.postID AND userID = ?1))
    )
ORDER BY p.postID DESC
LIMIT 20;
