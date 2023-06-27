CREATE TABLE postAllowedUser
(
    `postID`  INTEGER NOT NULL,
    `userID`  INTEGER NOT NULL,

    UNIQUE (postID, userID) ON CONFLICT REPLACE,
    FOREIGN KEY (postID) REFERENCES post (postID),
    FOREIGN KEY (userID) REFERENCES user (userID)
);
