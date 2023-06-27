SELECT e.*
FROM event e
         JOIN eventMember eM ON
            e.eventID = eM.eventID AND
            eM.userID = ?1 AND
            eM.status = 'GOING';
