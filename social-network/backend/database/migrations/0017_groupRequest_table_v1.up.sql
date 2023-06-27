CREATE TABLE groupRequest
(
    `groupID`  INTEGER NOT NULL,
    `senderID` INTEGER NOT NULL,

    FOREIGN KEY (groupID) REFERENCES `group` (groupID),
    FOREIGN KEY (senderID) REFERENCES user (userID),

    UNIQUE (groupID, senderID) ON CONFLICT REPLACE
);


DROP TRIGGER after_invite_accept;

CREATE TRIGGER after_invite_accept
    AFTER INSERT ON groupMember
BEGIN
    DELETE FROM groupInvite
    WHERE receiverID = NEW.userID AND groupID = NEW.groupID;

    DELETE FROM groupRequest
    WHERE groupID = NEW.groupID AND senderID = NEW.userID;
END;


DROP TRIGGER check_group_invite;
