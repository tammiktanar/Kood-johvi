package models

import (
	"database/sql"
	"fmt"
	"github.com/google/uuid"
	"io"
	"os"
	"path"
	"regexp"
	"social-network/queries"
	"time"
)

const PersistPath = "./persist"
const UploadsPath = PersistPath + "/uploads"

type File struct {
	Token     string    `json:"UUID"`
	Name      string    `json:"name"`
	Extension string    `json:"extension"`
	Created   time.Time `json:"created"`
}

func (x *File) pointerSlice() []interface{} {
	return []interface{}{
		&x.Token,
		&x.Name,
		&x.Extension,
		&x.Created,
	}
}

type FileModel struct {
	queries queries.QueryProvider
	db      *sql.DB
}

func MakeFileModel(db *sql.DB) FileModel {
	_ = os.Mkdir(UploadsPath, os.ModePerm)

	return FileModel{
		queries: queries.NewQueryProvider(db, "file"),
		db:      db,
	}
}

func (model FileModel) Get(token string) (*File, error) {
	stmt := model.queries.Prepare("get")

	row := stmt.QueryRow(token)

	file := &File{}
	err := row.Scan(file.pointerSlice()...)
	if err != nil {
		return nil, fmt.Errorf("File/Get: %w", err)
	}

	return file, nil
}

var extensionRegex = regexp.MustCompile(`\.\w+$`)

func (model FileModel) Insert(file io.Reader, filename string) (string, error) {
	stmt := model.queries.Prepare("insert")

	extension := extensionRegex.FindString(filename)

	token := uuid.New().String()

	out, err := os.Create(path.Join(UploadsPath, token+extension))
	if err != nil {
		return "", fmt.Errorf("File/Insert: %w", err)
	}
	defer out.Close()

	_, err = io.Copy(out, file)
	if err != nil {
		return "", fmt.Errorf("File/Insert: %w", err)
	}

	_, err = stmt.Exec(token, filename, extension)
	if err != nil {
		return "", fmt.Errorf("File/Insert: %w", err)
	}

	return token, err
}

func (model FileModel) Delete(token string) (bool, error) {
	stmt := model.queries.Prepare("delete")

	file, err := model.Get(token)
	if err != nil {
		return false, fmt.Errorf("File/Delete: %w", err)
	}

	res, err := stmt.Exec(token)
	if err != nil {
		return false, fmt.Errorf("File/Delete: %w", err)
	}

	err = os.Remove(path.Join(UploadsPath, token+file.Extension))
	if err != nil {
		return false, fmt.Errorf("File/Delete: %w", err)
	}

	n, _ := res.RowsAffected()

	return n > 0, nil
}
