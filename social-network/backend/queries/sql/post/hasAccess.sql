-- This is a mess, I'm sorry for anyone reading this >_<

WITH following AS (SELECT followingID FROM follow f WHERE f.followerID = ?1)

SELECT count(*) != 0
FROM post p
WHERE p.postID = ?2
  AND (
        (p.groupID IS NOT NULL) -- AND EXISTS(SELECT * FROM groupMember WHERE groupID = p.groupID AND userID = ?1))
        OR (p.groupID IS NULL AND (
                ?1 = p.authorID OR
                (p.privacy = 'public') OR -- AND ((SELECT private FROM user WHERE userID = p.authorID) = FALSE OR p.authorID IN following)) OR
                (p.privacy = 'private' AND p.authorID IN following) OR
                (p.privacy = 'manual' AND EXISTS(SELECT * FROM postAllowedUser WHERE postID = p.postID AND userID = ?1))
        ))
    );
