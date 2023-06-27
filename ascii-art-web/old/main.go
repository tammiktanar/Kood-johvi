package main

import (
	"html/template"
	"log"
	"net/http"
	"os"
	"strings"
)

type pageData struct {
	Title string
}

var tpl *template.Template

func init() {
	tpl = template.Must(template.ParseGlob("templates/*.gohtml"))
}

func main() {
	http.HandleFunc("/", index)
	http.HandleFunc("/process", process)

	http.Handle("/js/", http.StripPrefix("/js/", http.FileServer(http.Dir("js"))))
	http.Handle("/css/", http.StripPrefix("/css/", http.FileServer(http.Dir("css"))))

	http.ListenAndServe(":8080", nil)
}

func index(w http.ResponseWriter, req *http.Request) {

	pd := pageData{
		Title: "Ascii Art Web",
	}

	if !checkURL(req.URL.Path) {
		err := tpl.ExecuteTemplate(w, "404.gohtml", nil)

		if err != nil {
			log.Println(err)
			http.Error(w, "Internal server error", http.StatusInternalServerError)
			return
		}
	} else {

		err := tpl.ExecuteTemplate(w, "head.gohtml", nil)

		if err != nil {
			log.Println(err)
			http.Error(w, "Internal server error", http.StatusInternalServerError)
			return
		}
		err = tpl.ExecuteTemplate(w, "index.gohtml", pd)
		if err != nil {
			log.Println(err)
			http.Error(w, "Internal server error", http.StatusInternalServerError)
			return
		} else {
			err = tpl.ExecuteTemplate(w, "foot.gohtml", nil)

			if err != nil {
				log.Println(err)
				http.Error(w, "Internal server error", http.StatusInternalServerError)
				return
			}
		}
	}
}

func process(w http.ResponseWriter, req *http.Request) {
	stopHere := false

	if req.Method == http.MethodPost && req.URL.Path == "/process" {
		if req.PostFormValue("subAsciiArt") == "1" {
			if req.PostFormValue("textToConvert") != "" {
				stopHere = true
				index(w, req)
			} else {
				stopHere = true
				index(w, req)
			}
		}
	}

	if !stopHere {
		err := tpl.ExecuteTemplate(w, "head.gohtml", nil)
		if err != nil {
			log.Println(err)
			http.Error(w, "Internal server error", http.StatusInternalServerError)
			return
		} else {
			err = tpl.ExecuteTemplate(w, "index.gohtml", nil)
			if err != nil {
				log.Println(err)
				http.Error(w, "Internal server error", http.StatusInternalServerError)
				return
			} else {
				err = tpl.ExecuteTemplate(w, "foot.gohtml", nil)
			}
		}
	}
}

func apply(w http.ResponseWriter, req *http.Request) {
	err := tpl.ExecuteTemplate(w, "apply.gohtml", nil)
	if err != nil {
		log.Println(err)
		http.Error(w, "Internal server error", http.StatusInternalServerError)
	}
}

func checkURL(urlGiven string) bool {
	res := true
	check := ""
	if strings.Contains(urlGiven, ".gohtml") {
		check = strings.Split(urlGiven, ".gohtml")[0]
	} else {
		check = urlGiven
	}

	switch check {
	case "/index":
	case "/apply":
	case "/":
	case "/process":
	case "/favicon.ico":
	case "/static/js/bootstrap.bundle.min.js":
	case "/static/css/bootstrap.min.css":
	default:
		if _, err := os.Stat("/path/to/whatever"); os.IsNotExist(err) {
			res = false
		}
	}

	return res
}
