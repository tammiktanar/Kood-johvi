package query

import "database/sql"

type Count struct {
	Likes    int
	Dislikes int
}

/* Adds the count of likes and dislikes to a post */
func GetLikesDislike(db *sql.DB, postid int) (Count, error) {
	var counts Count
	// Check if post has any likes or dislikes
	var temp int
	if err := db.QueryRow("SELECT postid FROM postlikes WHERE postid = ?", postid).Scan(&temp); err == nil {

		q := "SELECT COUNT(like) FROM postlikes WHERE like = ? AND postid = ?"

		// Get the dislike count for the post
		var dislikeCount int
		if err := db.QueryRow(q, 0, postid).Scan(&dislikeCount); err == nil {
			counts.Dislikes = dislikeCount

		} else if err != sql.ErrNoRows {
			return counts, err
		}

		// Get the like count for the post
		var likeCount int
		if err := db.QueryRow(q, 1, postid).Scan(&likeCount); err == nil {
			counts.Likes = likeCount

		} else if err != sql.ErrNoRows {
			return counts, err
		}
	}

	return counts, nil
}
func GetTags(db *sql.DB, postid int) ([]string, error) {
	var tags []string

	rows, err := db.Query("SELECT tagid FROM posttags WHERE postid = ?", postid)
	if err != nil {
		return tags, err
	}

	for rows.Next() {
		var tagid string

		if err := rows.Scan(&tagid); err != nil {
			if err != sql.ErrNoRows {
				return tags, err
			}
		}

		var tagname string
		if err := db.QueryRow("SELECT name FROM tags WHERE id = ?", tagid).Scan(&tagname); err != nil {
			return tags, err
		}

		tags = append(tags, tagname)
	}

	if err := rows.Err(); err != nil {
		return tags, err
	}

	return tags, err
}
