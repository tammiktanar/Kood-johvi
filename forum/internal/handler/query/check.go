package query

import (
	"database/sql"
	"strconv"
)

func CheckPostLikes(db *sql.DB, postid string, userid int, isLike int) error {
	// Query for like value - 1 means user liked, 0 means user disliked
	row := db.QueryRow("SELECT like FROM postlikes WHERE userid = ? AND postid = ?", userid, postid)

	var likeVal int
	switch err := row.Scan(&likeVal); err {

	// If user doesnt have a previous reaction to that post we add a row to postlikes
	case sql.ErrNoRows:

		stmt, err := db.Prepare("INSERT INTO postlikes (postid, userid, like) VALUES (?, ?, ?)")
		if err != nil {
			return err
		}
		stmt.Exec(postid, userid, isLike)
	// If user has reacted to that post
	case nil:
		// User has reacted the same(liked or disliked again)
		if likeVal == isLike {
			stmt, err := db.Prepare("DELETE FROM postlikes WHERE postid = ? AND userid = ?")
			if err != nil {
				return err
			}
			stmt.Exec(postid, userid)

			// User reacted by liking or disliking
		} else {
			stmt, err := db.Prepare("UPDATE postlikes SET like = ? WHERE userid = ? AND postid = ?")
			if err != nil {
				return err
			}
			stmt.Exec(isLike, userid, postid)
		}

	// If something unexpected happened (an error)
	default:
		return err
	}

	return nil
}

func CheckCommentLikes(db *sql.DB, userid int, commentid string, isLike int) error {
	// Query for like value - 1 means user liked, 0 means user disliked
	row := db.QueryRow("SELECT like FROM commentlikes WHERE userid = ? AND commentid = ?", userid, commentid)

	var likeVal int
	switch err := row.Scan(&likeVal); err {

	// If user doesnt have a previous reaction to that comment we add a row to commentlikes
	case sql.ErrNoRows:
		stmt, err := db.Prepare("INSERT INTO commentlikes (commentid, userid, like) VALUES (?, ?, ?)")
		if err != nil {
			return err
		}
		stmt.Exec(commentid, userid, isLike)

	// If user has reacted to that post
	case nil:

		// User has reacted the same(liked or disliked again)
		if isLike == likeVal {
			stmt, err := db.Prepare("DELETE FROM commentlikes WHERE commentid = ? AND userid = ?")
			if err != nil {
				return err
			}
			stmt.Exec(commentid, userid)

			// User reacted by liking or disliking
		} else {
			stmt, err := db.Prepare("UPDATE commentlikes SET like = ? WHERE userid = ? AND commentid = ?")
			if err != nil {
				return err
			}
			stmt.Exec(isLike, userid, commentid)
		}

	// If something unexpected happened
	default:
		return err
	}

	return nil

}

func CheckURLQuery(db *sql.DB, q string, value string) error {
	id, err := strconv.Atoi(value)
	if err != nil {
		return err
	}

	if err := db.QueryRow(q, id).Scan(&id); err != nil {
		return err

	}

	return nil
}
