SELECT u.*
FROM "group" g
     JOIN groupMember gm ON g.groupID = gm.groupID
     JOIN user u ON gm.userID = u.userID
WHERE g.groupID = ?;
