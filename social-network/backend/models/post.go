package models

import (
	"database/sql"
	"fmt"
	"social-network/queries"
	"time"
)

type Post struct {
	PostID   int64  `json:"postID"`
	AuthorID int64  `json:"authorID"`
	GroupID  *int64 `json:"groupID"`

	Content string `json:"content"`
	Images  string `json:"images"`
	Privacy string `json:"privacy"`

	Created time.Time `json:"created"`

	Author   *UserLimited `json:"author,omitempty"`
	Group    *Group       `json:"group,omitempty"`
	Comments []Comment    `json:"comments,omitempty"`
}

func (x *Post) pointerSlice() []interface{} {
	return []interface{}{
		&x.PostID,
		&x.AuthorID,
		&x.GroupID,
		&x.Content,
		&x.Images,
		&x.Privacy,
		&x.Created,
	}
}

type PostModel struct {
	queries queries.QueryProvider
	db      *sql.DB
}

func MakePostModel(db *sql.DB) PostModel {
	return PostModel{
		queries: queries.NewQueryProvider(db, "post"),
		db:      db,
	}
}

func (model PostModel) Insert(post Post) (int64, error) {
	stmt := model.queries.Prepare("insert")

	res, err := stmt.Exec(post.pointerSlice()[:6]...)

	if err != nil {
		return 0, fmt.Errorf("Post/Insert: %w", err)
	}

	return res.LastInsertId()
}

func (model PostModel) GetByID(postID int64) (*Post, error) {
	stmt := model.queries.Prepare("getByID")

	row := stmt.QueryRow(postID)

	post := &Post{}
	author := &User{}
	err := row.Scan(append(post.pointerSlice(), author.pointerSlice()...)...)
	post.Author = author.Limited()

	if err != nil {
		return nil, fmt.Errorf("Post/GetByID: %w", err)
	}

	return post, nil
}

func (model PostModel) GetByUser(myID, targetID, beforeID int64) ([]*Post, error) {
	stmt := model.queries.Prepare("getByUser")

	rows, err := stmt.Query(myID, targetID, beforeID)
	defer rows.Close()
	if err != nil {
		return nil, fmt.Errorf("Post/GetByUser: %w", err)
	}

	posts := make([]*Post, 0)

	for rows.Next() {
		post := &Post{}
		author := &User{}
		err = rows.Scan(append(post.pointerSlice(), author.pointerSlice()...)...)
		post.Author = author.Limited()

		if err != nil {
			return nil, fmt.Errorf("Post/GetByUser: %w", err)
		}

		posts = append(posts, post)
	}

	return posts, nil
}

func (model PostModel) GetAll(myID, beforeID int64) ([]*Post, error) {
	stmt := model.queries.Prepare("getAll")

	rows, err := stmt.Query(myID, beforeID)
	defer rows.Close()
	if err != nil {
		return nil, fmt.Errorf("Post/GetAll: %w", err)
	}

	posts := make([]*Post, 0)

	for rows.Next() {
		post := &Post{}
		author := &User{}
		err = rows.Scan(append(post.pointerSlice(), author.pointerSlice()...)...)
		post.Author = author.Limited()

		if err != nil {
			return nil, fmt.Errorf("Post/GetAll: %w", err)
		}

		posts = append(posts, post)
	}

	return posts, nil
}

func (model PostModel) GetByFollowing(myID, beforeID int64) ([]*Post, error) {
	stmt := model.queries.Prepare("getByFollowing")

	rows, err := stmt.Query(myID, beforeID)
	defer rows.Close()
	if err != nil {
		return nil, fmt.Errorf("Post/GetByFollowing: %w", err)
	}

	posts := make([]*Post, 0)

	for rows.Next() {
		post := &Post{}
		author := &User{}
		err = rows.Scan(append(post.pointerSlice(), author.pointerSlice()...)...)
		post.Author = author.Limited()

		if err != nil {
			return nil, fmt.Errorf("Post/GetByFollowing: %w", err)
		}

		posts = append(posts, post)
	}

	return posts, nil
}

func (model PostModel) GetByGroup(groupID, beforeID int64) ([]*Post, error) {
	stmt := model.queries.Prepare("getByGroup")

	rows, err := stmt.Query(groupID, beforeID)
	defer rows.Close()
	if err != nil {
		return nil, fmt.Errorf("Post/GetByGroup: %w", err)
	}

	posts := make([]*Post, 0)

	for rows.Next() {
		post := &Post{}
		author := &User{}
		err = rows.Scan(append(post.pointerSlice(), author.pointerSlice()...)...)
		post.Author = author.Limited()

		if err != nil {
			return nil, fmt.Errorf("Post/GetByGroup: %w", err)
		}

		posts = append(posts, post)
	}

	return posts, nil
}

func (model PostModel) GetByMyGroups(myID, beforeID int64) ([]*Post, error) {
	stmt := model.queries.Prepare("getByMyGroups")

	rows, err := stmt.Query(myID, beforeID)
	defer rows.Close()
	if err != nil {
		return nil, fmt.Errorf("Post/GetByMyGroups: %w", err)
	}

	posts := make([]*Post, 0)

	for rows.Next() {
		post := &Post{}
		author := &User{}
		group := &Group{}

		err = rows.Scan(append(post.pointerSlice(), append(author.pointerSlice(), group.pointerSlice()...)...)...)

		post.Author = author.Limited()
		post.Group = group

		if err != nil {
			return nil, fmt.Errorf("Post/GetByMyGroups: %w", err)
		}

		posts = append(posts, post)
	}

	return posts, nil
}

func (model PostModel) HasAccess(userID, postID int64) (bool, error) {
	stmt := model.queries.Prepare("hasAccess")

	row := stmt.QueryRow(userID, postID)

	var access bool
	err := row.Scan(&access)

	if err != nil {
		return false, fmt.Errorf("Post/HasAccess: %w", err)
	}

	return access, nil
}

func (model PostModel) InsertAllowedUser(postID, userID int64) error {
	stmt := model.queries.Prepare("insertAllowedUser")

	_, err := stmt.Exec(postID, userID)

	if err != nil {
		return fmt.Errorf("Post/InsertAllowedUser: %w", err)
	}

	return nil
}
