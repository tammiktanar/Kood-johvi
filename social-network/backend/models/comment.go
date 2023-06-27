package models

import (
	"database/sql"
	"fmt"
	"social-network/queries"
	"time"
)

type Comment struct {
	CommentID int64 `json:"commentID"`
	PostID    int64 `json:"postID"`
	AuthorID  int64 `json:"authorID"`

	Content string `json:"content"`
	Images  string `json:"images"`

	Created time.Time `json:"created"`

	Author *UserLimited `json:"author,omitempty"`
}

func (x *Comment) pointerSlice() []interface{} {
	return []interface{}{
		&x.CommentID,
		&x.PostID,
		&x.AuthorID,
		&x.Content,
		&x.Images,
		&x.Created,
	}
}

type CommentModel struct {
	queries queries.QueryProvider
	db      *sql.DB
}

func MakeCommentModel(db *sql.DB) CommentModel {
	return CommentModel{
		queries: queries.NewQueryProvider(db, "comment"),
		db:      db,
	}
}

func (model CommentModel) GetByID(id int64) (*Comment, error) {
	stmt := model.queries.Prepare("getByID")

	row := stmt.QueryRow(id)

	comment := &Comment{}
	err := row.Scan(comment.pointerSlice()...)

	if err != nil {
		return nil, fmt.Errorf("Comment/GetByID: %w", err)
	}

	return comment, nil
}

func (model CommentModel) GetByPost(postID int64) ([]*Comment, error) {
	stmt := model.queries.Prepare("getByPost")

	rows, err := stmt.Query(postID)
	defer rows.Close()
	if err != nil {
		return nil, fmt.Errorf("Comment/GetByID: %w", err)
	}

	comments := make([]*Comment, 0)

	for rows.Next() {
		comment := &Comment{}
		user := &User{}

		err = rows.Scan(append(comment.pointerSlice(), user.pointerSlice()...)...)
		if err != nil {
			return nil, fmt.Errorf("Comment/GetByID: %w", err)
		}

		comment.Author = user.Limited()

		comments = append(comments, comment)
	}

	return comments, nil
}

func (model CommentModel) Insert(comment Comment) (int64, error) {
	stmt := model.queries.Prepare("insert")

	res, err := stmt.Exec(
		comment.PostID,
		comment.AuthorID,
		comment.Content,
		comment.Images,
	)

	if err != nil {
		return 0, fmt.Errorf("Comment/Insert: %w", err)
	}

	return res.LastInsertId()
}
