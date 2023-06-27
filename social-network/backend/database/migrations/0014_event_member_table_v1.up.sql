CREATE TABLE eventMember
(
    `eventID` INTEGER NOT NULL,
    `userID`  INTEGER NOT NULL,
    `status`  TEXT    NOT NULL CHECK ( status IN ('GOING', 'NOT_GOING')),

    UNIQUE (eventID, userID) ON CONFLICT REPLACE,
    FOREIGN KEY (eventID) REFERENCES event (eventID),
    FOREIGN KEY (userID) REFERENCES user (userID)
);

-- This trigger checks if the inserted status is null.
-- If so, remove the matching row instead of inserting.
CREATE TRIGGER eventMember_remove_if_null
    BEFORE INSERT ON eventMember
    WHEN NEW.status IS NULL
BEGIN
    -- Delete existing row if status is null.
    DELETE FROM eventMember
    WHERE eventID = NEW.eventID AND userID = NEW.userID;

    -- And don't insert the given row.
    SELECT RAISE(IGNORE);
END;

-- If a user leaves a group, remove them from that group's events
CREATE TRIGGER group_leave_events
    AFTER DELETE ON groupMember
BEGIN
    DELETE FROM eventMember
    WHERE (SELECT groupID FROM event e WHERE e.eventID = eventMember.eventID) = OLD.groupID AND
          eventMember.userID = OLD.userID;
END;
