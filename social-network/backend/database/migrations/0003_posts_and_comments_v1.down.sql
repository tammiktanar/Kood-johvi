PRAGMA foreign_keys= off;

CREATE TABLE `post_new`
(
    `postID`   INTEGER PRIMARY KEY AUTOINCREMENT,
    `authorID` INTEGER NOT NULL,
    `groupID`  INTEGER,

    `content`  TEXT    NOT NULL,
    `images`   TEXT    NOT NULL DEFAULT '',

    `created`  DATE    NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (authorID) REFERENCES user (userID),
    FOREIGN KEY (groupID) REFERENCES `group` (groupID)
);

INSERT INTO post_new (postID, authorID, groupID, content, images, created)
SELECT postID, authorID, groupID, content, images, created
FROM post;

DROP TABLE post;

ALTER TABLE post_new
    RENAME TO post;

CREATE INDEX IF NOT EXISTS post_index_author
    ON post (authorID);

CREATE INDEX IF NOT EXISTS post_index_group
    ON post (groupID);

PRAGMA foreign_keys= on;
