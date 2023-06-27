SELECT *,
       EXISTS(SELECT * FROM follow f WHERE f.followerID = ?2 AND f.followingID = ?1) AS meToYou,
       EXISTS(SELECT * FROM followRequest fr WHERE fr.followerID = ?2 AND fr.followingID = ?1) AS meToYouPending,
       EXISTS(SELECT * FROM followRequest fr WHERE fr.followerID = ?1 AND fr.followingID = ?2) AS youToMePending
FROM user
WHERE userID = ?1
