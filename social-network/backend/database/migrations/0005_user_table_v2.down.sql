PRAGMA foreign_keys= off;

CREATE TABLE `user_new`
(
    `userID`    INTEGER PRIMARY KEY AUTOINCREMENT,

    `email`     TEXT NOT NULL UNIQUE COLLATE NOCASE,
    `password`  TEXT NOT NULL,

    `firstname` TEXT NOT NULL,
    `lastname`  TEXT NOT NULL,

    `created`   DATE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO user_new (userID, email, password, firstname, lastname, created)
SELECT userID, email, password, firstname, lastname, created
FROM user;

DROP TABLE user;

ALTER TABLE user_new
    RENAME TO user;

PRAGMA foreign_keys= on;
