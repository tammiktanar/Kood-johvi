package main

import (
	"database/sql"
	"forum/internal/env" // imports Env struct, where we store the db connection
	"forum/internal/handler"
	"forum/internal/handler/auth"
	"log"
	"net/http"

	_ "github.com/mattn/go-sqlite3"
)

func main() {
	db, err := sql.Open("sqlite3", "./db/storage.db")
	if err != nil {
		log.Fatal(err)
	}
	// Makes an environment for Database connection
	env := &env.Env{DB: db}

	// One crate statement for the audit
	db.Exec(`CREATE TABLE IF NOT EXISTS "postlikes" (
		"postid"	INTEGER NOT NULL,
		"userid"	INTEGER NOT NULL,
		"like"	INTEGER NOT NULL,
		FOREIGN KEY("postid") REFERENCES "posts"("postid") ON DELETE CASCADE ON UPDATE CASCADE,
		FOREIGN KEY("userid") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE,
		PRIMARY KEY("userid","postid")
	);`)

	http.HandleFunc("/", handler.Home(env))
	http.HandleFunc("/createpost", handler.CreatePost(env))
	http.HandleFunc("/post", handler.ViewPost(env))
	http.HandleFunc("/addcomment", handler.AddComment(env))

	http.HandleFunc("/search", handler.Search(env))
	http.HandleFunc("/react", handler.React(env))

	http.HandleFunc("/register", auth.Register())
	http.HandleFunc("/registerauth", auth.RegisterAuth(env))
	http.HandleFunc("/login", auth.Login(env))
	http.HandleFunc("/loginauth", auth.LoginAuth(env))
	http.HandleFunc("/logout", auth.Logout(env))

	http.HandleFunc("/user", handler.UserDetails(env))

	fs := http.FileServer(http.Dir("./assets"))
	http.Handle("/assets/", http.StripPrefix("/assets/", fs))

	http.HandleFunc("/favicon.ico", ignoreFavicon)
	if err := http.ListenAndServe(":8000", nil); err != nil {
		log.Fatal(err)
	}

}

func ignoreFavicon(w http.ResponseWriter, r *http.Request) {}
