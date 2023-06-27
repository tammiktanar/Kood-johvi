DROP TABLE groupRequest;


DROP TRIGGER after_invite_accept;

CREATE TRIGGER after_invite_accept
    AFTER INSERT ON groupMember
BEGIN
    DELETE FROM groupInvite
    WHERE receiverID = NEW.userID AND groupID = NEW.groupID;
END;


CREATE TRIGGER check_group_invite
    BEFORE INSERT ON groupMember
    WHEN EXISTS(SELECT * FROM "groupMember" gm WHERE gm.groupID = NEW.groupID)
        AND NOT EXISTS(SELECT * FROM "group" g WHERE g.groupID = NEW.groupID AND g.type = 'public')
        AND NOT EXISTS(SELECT * FROM groupInvite gi WHERE gi.groupID = NEW.groupID AND gi.receiverID = NEW.userID)
BEGIN
    SELECT RAISE(ROLLBACK, 'User does not have an invite to this private group');
END;
