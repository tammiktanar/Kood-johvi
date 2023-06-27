DELETE
FROM follow
WHERE followerID = ?1
  AND followingID = ?2;
