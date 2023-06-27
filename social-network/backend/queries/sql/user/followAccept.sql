INSERT INTO follow (followerID, followingID)
SELECT *
FROM followRequest
WHERE followerID = ?2
  AND followingID = ?1;
