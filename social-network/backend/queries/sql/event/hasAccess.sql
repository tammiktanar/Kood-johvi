SELECT g.type = 'public' OR EXISTS(SELECT * FROM groupMember gm WHERE gm.groupID = g.groupID AND gm.userID = ?2) AS hasAccess
FROM event e JOIN "group" g ON e.groupID = g.groupID
WHERE e.eventID = ?1;
