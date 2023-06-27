SELECT u.*, eM.status
FROM event e
         JOIN eventMember eM ON e.eventID = eM.eventID
         JOIN user u ON eM.userID = u.userID
WHERE e.eventID = ?1;
