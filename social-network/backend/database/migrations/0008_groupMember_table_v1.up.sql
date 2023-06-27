CREATE TABLE groupMember
(
    `groupID` INTEGER NOT NULL,
    `userID`  INTEGER NOT NULL,

    FOREIGN KEY (groupID) REFERENCES `group` (groupID),
    FOREIGN KEY (userID) REFERENCES user (userID),

    UNIQUE (groupID, userID)
);

CREATE INDEX groupMember_reverse
    ON groupMember (userID, groupID);

CREATE TRIGGER check_group_invite
    BEFORE INSERT ON groupMember
    WHEN EXISTS(SELECT * FROM "groupMember" gm WHERE gm.groupID = NEW.groupID)
        AND NOT EXISTS(SELECT * FROM "group" g WHERE g.groupID = NEW.groupID AND g.type = 'public')
        AND NOT EXISTS(SELECT * FROM groupInvite gi WHERE gi.groupID = NEW.groupID AND gi.receiverID = NEW.userID)
    BEGIN
        SELECT RAISE(ROLLBACK, 'User does not have an invite to this private group');
    END;

CREATE TRIGGER after_invite_accept
    AFTER INSERT ON groupMember
    BEGIN
        DELETE FROM groupInvite
        WHERE receiverID = NEW.userID AND groupID = NEW.groupID;
    END;

CREATE TRIGGER group_owner_leave
    BEFORE DELETE ON groupMember
    WHEN (SELECT ownerID FROM "group" g WHERE g.groupID = OLD.groupID) = OLD.userID
    BEGIN
        SELECT RAISE(ROLLBACK, 'Owner of the group cannot leave');
    END;
