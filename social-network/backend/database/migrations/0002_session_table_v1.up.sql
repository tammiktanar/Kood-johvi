CREATE TABLE `session`
(
    `token`   TEXT PRIMARY KEY,
    `userID`  INTEGER NOT NULL,

    `created` DATE    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `expires` DATE    NOT NULL,

    FOREIGN KEY (userID) REFERENCES user (userID)
);
