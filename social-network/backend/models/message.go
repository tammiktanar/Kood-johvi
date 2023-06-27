package models

import (
	"database/sql"
	"fmt"
	"social-network/queries"
	"time"
)

type Message struct {
	MessageID int64     `json:"messageID"`
	Sender    int64     `json:"sender"`
	Receiver  int64     `json:"receiver"`
	Content   string    `json:"content"`
	Created   time.Time `json:"created"`

	IsGroup    bool         `json:"isGroup"`
	SenderData *UserLimited `json:"senderData"`
}

func (x *Message) pointerSlice() []interface{} {
	return []interface{}{
		&x.MessageID,
		&x.Sender,
		&x.Receiver,
		&x.Content,
		&x.Created,
	}
}

type MessageModel struct {
	queries queries.QueryProvider
	db      *sql.DB
}

func MakeMessageModel(db *sql.DB) MessageModel {
	return MessageModel{
		queries: queries.NewQueryProvider(db, "message"),
		db:      db,
	}
}

func (model *MessageModel) SendMessage(message Message) (int64, error) {
	var stmt *sql.Stmt
	if message.IsGroup {
		stmt = model.queries.Prepare("groupSendMessage")
	} else {
		stmt = model.queries.Prepare("userSendMessage")
	}

	res, err := stmt.Exec(
		message.pointerSlice()[:4]...,
	)

	if err != nil {
		return 0, fmt.Errorf("Message/SendMessage: %w", err)
	}

	return res.LastInsertId()
}

func (model *MessageModel) GetMessages(messageOld Message) ([]*Message, error) {
	var stmt *sql.Stmt
	if messageOld.IsGroup {
		stmt = model.queries.Prepare("groupGetMessages")
	} else {
		stmt = model.queries.Prepare("userGetMessages")
	}

	rows, err := stmt.Query(messageOld.Sender, messageOld.Receiver, messageOld.MessageID)
	defer rows.Close()
	if err != nil {
		return nil, fmt.Errorf("Message/GetMessages: %w", err)
	}

	messages := make([]*Message, 0)

	for rows.Next() {
		message := &Message{}
		pointers := message.pointerSlice()
		if messageOld.IsGroup {
			user := &UserLimited{}
			message.SenderData = user
			pointers = append(pointers, &user.UserID, &user.FirstName, &user.LastName, &user.Nickname, &user.Image)
		}
		err = rows.Scan(pointers...)

		if err != nil {
			return nil, fmt.Errorf("Message/GetMessages: %w", err)
		}

		messages = append(messages, message)
	}

	return messages, nil
}
