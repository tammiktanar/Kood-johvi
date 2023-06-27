CREATE TABLE `post`
(
    `postID`   INTEGER PRIMARY KEY AUTOINCREMENT,
    `authorID` INTEGER NOT NULL,

    `content`  TEXT    NOT NULL,
    `images`   TEXT    NOT NULL DEFAULT '',

    `created`  DATE    NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (authorID) REFERENCES user (userID)
);

CREATE TABLE `comment`
(
    `commentID` INTEGER PRIMARY KEY AUTOINCREMENT,
    `postID`    INTEGER NOT NULL,
    `authorID`  INTEGER NOT NULL,

    `content`   TEXT    NOT NULL,

    `created`   DATE    NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (postID) REFERENCES post (postID),
    FOREIGN KEY (authorID) REFERENCES user (userID)
);

CREATE INDEX IF NOT EXISTS comment_index
    ON comment (postID);
