CREATE TABLE messageUser
(
    `messageID` INTEGER PRIMARY KEY AUTOINCREMENT,
    `sender`    INTEGER NOT NULL,
    `receiver`  INTEGER NOT NULL,
    `content`   TEXT    NOT NULL,
    `created`   DATE    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (sender) REFERENCES user (userID),
    FOREIGN KEY (receiver) REFERENCES user (userID)
);

CREATE INDEX messageUser_SR
    ON messageUser (sender, receiver);

CREATE INDEX messageUser_RS
    ON messageUser (receiver, sender);


CREATE TABLE messageGroup
(
    `messageID` INTEGER PRIMARY KEY AUTOINCREMENT,
    `sender`    INTEGER NOT NULL,
    `groupID`   INTEGER NOT NULL,
    `content`   TEXT    NOT NULL,
    `created`   DATE    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (sender) REFERENCES user (userID),
    FOREIGN KEY (groupID) REFERENCES `group` (groupID)
);

CREATE INDEX messageGroup_SR
    ON messageGroup (sender, groupID);

CREATE INDEX messageGroup_RS
    ON messageGroup (groupID, sender);
