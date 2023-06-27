-- Added various fields for profile data

PRAGMA foreign_keys= off;

CREATE TABLE `user_new`
(
    `userID`    INTEGER PRIMARY KEY AUTOINCREMENT,

    `email`     TEXT NOT NULL UNIQUE COLLATE NOCASE,
    `password`  TEXT NOT NULL,

    `firstname` TEXT NOT NULL,
    `lastname`  TEXT NOT NULL,
    `nickname`  TEXT NOT NULL DEFAULT '',

    `created`   DATE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    `image`     TEXT,
    `about`     TEXT NOT NULL DEFAULT '',
    `birthday`  DATE NOT NULL,

    FOREIGN KEY (image) REFERENCES file (token)
);

INSERT INTO user_new (userID, email, password, firstname, nickname, lastname, created, image, about, birthday)
SELECT userID, email, password, firstname, '', lastname, created, null, '', CURRENT_TIMESTAMP
FROM user;

DROP TABLE user;

ALTER TABLE user_new
    RENAME TO user;

PRAGMA foreign_keys= on;
