SELECT count(*) != 0 AS isFollowing
FROM follow
WHERE followerID = ?1
  AND followingID = ?2;
