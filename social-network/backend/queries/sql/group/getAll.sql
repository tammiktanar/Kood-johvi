SELECT *,
       EXISTS(SELECT * FROM groupMember gm WHERE gm.groupID = g.groupID AND gm.userID = ?1 ) as includesMe,
       EXISTS(SELECT * FROM groupRequest gr WHERE gr.groupID = g.groupID AND gr.senderID = ?1 ) as pendingRequest
FROM "group" g
WHERE g.type = 'public' OR includesMe = TRUE;
