-- SELECT EXISTS(SELECT * FROM "group" g WHERE g.groupID = ?1 AND g.type = 'public')
--     OR EXISTS(SELECT * FROM groupInvite gi WHERE gi.groupID = ?1 AND gi.receiverID = ?2);

SELECT EXISTS(SELECT * FROM groupInvite gi WHERE gi.groupID = ?1 AND gi.receiverID = ?2);
