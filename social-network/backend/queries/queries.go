package queries

import (
	"database/sql"
	"embed"
	"fmt"
	"io/fs"
	"path"
	"strings"
)

// QueryProvider stores a map of queries as strings and also gives access to the database directly.
// The map keys correspond to sql file names, but without the ".sql" part.
type QueryProvider struct {
	db      *sql.DB
	queries map[string]string

	root string // Just for error msg
}

// NewQueryProvider makes a new query provider, given a database and a root path.
// For example, a root path of "user" would retrieve all ".sql" files in "social-network/backend/queries/sql/user/".
func NewQueryProvider(db *sql.DB, root string) QueryProvider {
	return QueryProvider{
		db:      db,
		queries: makeCommandMap(root),
		root:    root,
	}
}

// GetString will return the raw query as a string.
// The key corresponds to a file name, but without ".sql".
// It panics if it can't find the query.
func (qp QueryProvider) GetString(key string) string {
	query, ok := qp.queries[key]
	if !ok {
		panic(fmt.Errorf(`no such sql file: queries/sql/%v/%v.sql`, qp.root, key))
	}

	return query
}

// Prepare will prepare an SQL statement.
// The key corresponds to a file name, but without ".sql".
// It panics if it fails to prepare the statement.
func (qp QueryProvider) Prepare(key string) *sql.Stmt {
	query := qp.GetString(key)

	stmt, err := qp.db.Prepare(query)
	if err != nil {
		panic(fmt.Errorf(`error preparing queries/sql/%v/%v.sql: %w`, qp.root, key, err))
	}

	return stmt
}

//go:embed sql/*
var queryFiles embed.FS

func makeCommandMap(root string) map[string]string {
	dirs, _ := queryFiles.ReadDir(".")
	base := dirs[0].Name()

	ff, err := fs.Sub(queryFiles, path.Join(base, root))
	if err != nil {
		panic(fmt.Errorf("invalid root: %v", root))
	}

	matches, err := fs.Glob(ff, "*.sql")
	if err != nil {
		panic(err)
	}

	m := matchesToMap(ff, matches)

	return m
}

func matchesToMap(ff fs.FS, matches []string) map[string]string {
	m := make(map[string]string)

	for _, match := range matches {
		content, err := fs.ReadFile(ff, match)
		if err != nil {
			panic(err)
		}

		key := strings.TrimSuffix(match, ".sql")
		m[key] = string(content)
	}

	return m
}
