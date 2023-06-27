CREATE TABLE event
(
    `eventID`  INTEGER PRIMARY KEY AUTOINCREMENT,
    `groupID` INTEGER NOT NULL,
    `authorID` INTEGER NOT NULL,

    `title` TEXT NOT NULL,
    `about` TEXT NOT NULL,
    `time`  DATE NOT NULL,
    `created` DATE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (groupID) REFERENCES "group" (groupID),
    FOREIGN KEY (authorID) REFERENCES user (userID)
);
