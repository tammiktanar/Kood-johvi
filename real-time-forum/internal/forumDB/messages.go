package forumDB

import (
	"database/sql"
	"fmt"
	"time"
)

type Message struct {
	MessageID int       `json:"message_id"`
	Sender    int       `json:"sender"`
	Receiver  int       `json:"receiver"`
	Text      string    `   json:"text"`
	Date      time.Time `json:"date"`
}

type MessageModel struct {
	db         *sql.DB
	statements map[string]*sql.Stmt
}

func NewMessageModel(db *sql.DB) MessageModel {
	model := MessageModel{db: db}

	model.statements = makeStatementMap(db, "server/db/sql/models/messages.sql")

	return model
}

func (m MessageModel) Send(sender, receiver int, content string) (Message, error) {
	stmt := m.statements["Send"]

	t := time.Now()
	res, err := stmt.Exec(
		sender,
		receiver,
		content,
		t,
	)
	if err != nil {
		return Message{}, fmt.Errorf("message send: %w", err)
	}

	id, err := res.LastInsertId()
	if err != nil {
		return Message{}, fmt.Errorf("message send: %w", err)
	}

	message := Message{
		MessageID: int(id),
		Sender:    sender,
		Receiver:  receiver,
		Date:      t,
		Text:      content,
	}

	return message, nil
}

func (m MessageModel) Delete(messageID int) error {
	stmt := m.statements["Delete"]

	_, err := stmt.Exec(
		messageID,
	)

	return fmt.Errorf("ping delete: %w", err)
}

func (m MessageModel) GetHistory(sender, receiver, fromMessage int) (messages []Message, _ error) {
	stmt := m.statements["GetHistory"]

	rows, err := stmt.Query(sender, receiver, fromMessage)
	if err != nil {
		return nil, err
	}

	for rows.Next() {
		var message Message

		err = rows.Scan(
			&message.MessageID,
			&message.Sender,
			&message.Receiver,
			&message.Text,
			&message.Date,
		)
		if err != nil {
			return nil, err
		}

		messages = append(messages, message)
	}

	return messages, nil
}

func (m MessageModel) KnownUsers(userID int) (users []User, _ error) {
	stmt := m.statements["KnownUsers"]

	rows, err := stmt.Query(userID)
	if err != nil {
		return nil, err
	}

	for rows.Next() {
		var user User

		err = rows.Scan(
			&user.UserID,
			&user.Name,
			&user.Email,
			&user.Password,
			&user.Image,
			&user.Description,
			&user.Creation,
			&user.Birthday,
			&user.Gender,
			&user.FirstName,
			&user.LastName,
		)
		if err != nil {
			return nil, err
		}

		users = append(users, user)
	}

	return users, nil
}
