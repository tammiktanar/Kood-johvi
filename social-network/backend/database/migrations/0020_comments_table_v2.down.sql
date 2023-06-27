PRAGMA foreign_keys= off;

CREATE TABLE `comment_new`
(
    `commentID` INTEGER PRIMARY KEY AUTOINCREMENT,
    `postID`    INTEGER NOT NULL,
    `authorID`  INTEGER NOT NULL,

    `content`   TEXT    NOT NULL,

    `created`   DATE    NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (postID) REFERENCES post (postID),
    FOREIGN KEY (authorID) REFERENCES user (userID)
);

INSERT INTO comment_new (commentID, postID, authorID, content, created)
SELECT commentID, postID, authorID, content, created
FROM comment;

DROP TABLE comment;

ALTER TABLE comment_new
    RENAME TO comment;

CREATE INDEX IF NOT EXISTS comment_index
    ON comment (postID);

PRAGMA foreign_keys= on;
