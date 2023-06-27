CREATE TABLE `file`
(
    `token`      TEXT PRIMARY KEY,
    `name`      TEXT NOT NULL,
    `extension` TEXT NOT NULL,
    `created`   DATE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
