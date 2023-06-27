SELECT u.* FROM follow f
JOIN user u ON f.followerID = u.userID
WHERE f.followingID = ?;
