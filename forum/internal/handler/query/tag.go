package query

import "database/sql"

func GetAllTags(db *sql.DB) ([]string, error) {
	rows, err := db.Query("SELECT name FROM tags")
	if err != nil {
		return nil, err
	}

	defer rows.Close()

	var allTags []string

	for rows.Next() {

		var tag string
		if err := rows.Scan(&tag); err != nil {
			return allTags, err
		}

		allTags = append(allTags, tag)
	}

	if err = rows.Err(); err != nil {
		return allTags, err
	}

	return allTags, nil
}
