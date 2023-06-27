SELECT g.type = 'public' OR EXISTS(SELECT * FROM groupMember gm WHERE gm.groupID = ?1 AND gm.userID = ?2) AS hasAccess
FROM "group" g
WHERE g.groupID = ?1;
