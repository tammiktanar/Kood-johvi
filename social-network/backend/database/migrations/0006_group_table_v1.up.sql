CREATE TABLE `group`
(
    `groupID` INTEGER PRIMARY KEY AUTOINCREMENT,
    `ownerID` INTEGER NOT NULL,
    `name`    TEXT    NOT NULL,
    `about`   TEXT    NOT NULL,
    `image`   TEXT,
    `type`    TEXT    NOT NULL,
    `created` DATE    NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CHECK (type IN ('public', 'private')),
    FOREIGN KEY (ownerID) REFERENCES user (userID),
    FOREIGN KEY (image) REFERENCES file (token)
);
