-- Func: Send
INSERT INTO messages(sender, receiver, content, date)
values (?, ?, ?, ?);

-- Func: Delete
DELETE
FROM messages
WHERE messageID = ?;

-- Func: GetHistory
WITH var AS (SELECT ? AS sender, ? AS receiver, ? AS fromMsg)
SELECT *
FROM messages
WHERE ((select fromMsg from var) = 0 OR messageID < (select fromMsg from var))
  AND ((sender = (select sender from var) AND receiver = (select receiver from var))
    OR (sender = (select receiver from var) AND receiver = (select sender from var)))
ORDER BY messageID DESC
LIMIT 10;

-- Func: KnownUsers
WITH var AS (SELECT ? AS target),
     known AS (
         SELECT CASE WHEN sender != (SELECT target FROM var) THEN sender ELSE receiver END knownID
         FROM messages
         WHERE sender = (SELECT target FROM var)
            OR receiver = (SELECT target FROM var)
         GROUP BY knownID
         ORDER BY max(messageID) DESC
     )
SELECT u.* -- u.userID, u.name, u.email, '' AS password, u.image, u.description, u.created, u.birthday, u.gender, u.firstName, u.lastName
FROM known
         JOIN users AS u on userID = knownID;
