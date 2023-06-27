CREATE TABLE followRequest (
    `followerID` INTEGER NOT NULL,
    `followingID` INTEGER NOT NULL,

    UNIQUE (followerID, followingID) ON CONFLICT REPLACE,
    CHECK (followerID != followingID),
    FOREIGN KEY (followerID) REFERENCES user (userID),
    FOREIGN KEY (followingID) REFERENCES user (userID)
);

CREATE TRIGGER followRequest_check_follow
    BEFORE INSERT ON followRequest
    WHEN EXISTS(SELECT * FROM follow WHERE followerID = NEW.followerID AND followingID = NEW.followingID)
BEGIN
    SELECT RAISE(IGNORE);
END;
