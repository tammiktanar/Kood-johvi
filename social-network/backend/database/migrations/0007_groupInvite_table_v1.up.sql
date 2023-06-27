CREATE TABLE groupInvite
(
    `groupID` INTEGER NOT NULL,
    `senderID`  INTEGER NOT NULL,
    `receiverID`  INTEGER NOT NULL,

    FOREIGN KEY (groupID) REFERENCES `group` (groupID),
    FOREIGN KEY (senderID) REFERENCES user (userID),
    FOREIGN KEY (receiverID) REFERENCES user (userID),

    UNIQUE (receiverID, groupID, senderID) ON CONFLICT REPLACE
);
