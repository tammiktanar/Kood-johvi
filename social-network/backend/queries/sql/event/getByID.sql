SELECT e.*, IFNULL(eM.status, 'UNSET')
FROM event e
         LEFT JOIN eventMember eM on e.eventID = eM.eventID AND eM.userID = ?2
WHERE e.eventID = ?1;
