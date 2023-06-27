SELECT g.*
FROM "group" g
    JOIN groupMember gM on g.groupID = gM.groupID AND gM.userID = ?1;
