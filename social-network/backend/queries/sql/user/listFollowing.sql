SELECT u.* FROM follow f
JOIN user u ON f.followingID = u.userID
WHERE f.followerID = ?;
