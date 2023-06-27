CREATE TABLE follow
(
    `followerID`  INTEGER NOT NULL,
    `followingID` INTEGER NOT NULL,

    UNIQUE (followerID, followingID) ON CONFLICT REPLACE,
    CHECK (followerID != followingID),
    FOREIGN KEY (followerID) REFERENCES user (userID),
    FOREIGN KEY (followingID) REFERENCES user (userID)
);

CREATE TRIGGER follow_accept_request
    AFTER INSERT
    ON follow
BEGIN
    DELETE
    FROM followRequest
    WHERE followingID = NEW.followingID
      AND followerID = NEW.followerID;
END;
