SELECT EXISTS(SELECT * FROM groupRequest WHERE groupID = ?1 AND senderID = ?2);
