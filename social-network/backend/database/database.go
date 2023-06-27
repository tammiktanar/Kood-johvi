package database

import (
	"social-network/models"
)

type Database struct {
	User    models.UserModel
	Session models.SessionModel
	Post    models.PostModel
	Comment models.CommentModel
	File    models.FileModel
	Group   models.GroupModel
	Event   models.EventModel
	Message models.MessageModel
}

func NewDatabase(path string) *Database {
	db := openDB(path)

	database := &Database{
		User:    models.MakeUserModel(db),
		Session: models.MakeSessionModel(db),
		Post:    models.MakePostModel(db),
		Comment: models.MakeCommentModel(db),
		File:    models.MakeFileModel(db),
		Group:   models.MakeGroupModel(db),
		Event:   models.MakeEventModel(db),
		Message: models.MakeMessageModel(db),
	}

	return database
}
