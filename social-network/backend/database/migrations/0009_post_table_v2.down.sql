PRAGMA foreign_keys= off;

CREATE TABLE `post_new`
(
    `postID`   INTEGER PRIMARY KEY AUTOINCREMENT,
    `authorID` INTEGER NOT NULL,

    `content`  TEXT    NOT NULL,
    `images`   TEXT    NOT NULL DEFAULT '',

    `created`  DATE    NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (authorID) REFERENCES user (userID)
);

INSERT INTO post_new (postID, authorID, content, images, created)
SELECT postID, authorID, content, images, created
FROM post;

DROP TABLE post;

ALTER TABLE post_new
    RENAME TO post;

PRAGMA foreign_keys= on;
