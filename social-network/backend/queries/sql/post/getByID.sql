SELECT p.*, u.*
FROM post p
         JOIN user u on u.userID = p.authorID
WHERE postID = ?1;
