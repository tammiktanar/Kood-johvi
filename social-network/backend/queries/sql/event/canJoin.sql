SELECT EXISTS(SELECT * FROM groupMember gm WHERE gm.groupID = g.groupID AND gm.userID = ?2) AS canJoin
FROM event e JOIN "group" g ON e.groupID = g.groupID
WHERE e.eventID = ?1;
