-- Added groupID

PRAGMA foreign_keys= off;

CREATE TABLE `post_new`
(
    `postID`   INTEGER PRIMARY KEY AUTOINCREMENT,
    `authorID` INTEGER NOT NULL,
    `groupID`  INTEGER,

    `content`  TEXT    NOT NULL,
    `images`   TEXT    NOT NULL DEFAULT '',

    `privacy`  TEXT NOT NULL CHECK (`privacy` IN ('public', 'private', 'manual')),

    `created`  DATE    NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (authorID) REFERENCES user (userID),
    FOREIGN KEY (groupID) REFERENCES `group` (groupID)
);

INSERT INTO post_new (postID, authorID, groupID, content, images, privacy, created)
SELECT postID, authorID, groupID, content, images, 'public', created
FROM post;

DROP TABLE post;

ALTER TABLE post_new
    RENAME TO post;

-- Add a trigger that checks if the author of a group post has permission to the group
CREATE TRIGGER group_post_permission_check
    BEFORE INSERT
    ON post
    WHEN NEW.groupID IS NOT NULL AND NOT EXISTS(SELECT * FROM groupMember WHERE groupID = NEW.groupID AND userID = NEW.authorID)
BEGIN
    SELECT RAISE(ROLLBACK, 'Attempted to create a post in a group this user is not part of');
END;

CREATE INDEX IF NOT EXISTS post_index_author
    ON post (authorID);

CREATE INDEX IF NOT EXISTS post_index_group
    ON post (groupID);

PRAGMA foreign_keys= on;
