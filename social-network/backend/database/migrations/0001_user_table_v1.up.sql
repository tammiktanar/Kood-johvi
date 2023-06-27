CREATE TABLE `user`
(
    `userID`    INTEGER PRIMARY KEY AUTOINCREMENT,

    `email`     TEXT NOT NULL UNIQUE COLLATE NOCASE,
    `password`  TEXT NOT NULL,

    `firstname` TEXT NOT NULL,
    `lastname`  TEXT NOT NULL,

    `created`   DATE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
