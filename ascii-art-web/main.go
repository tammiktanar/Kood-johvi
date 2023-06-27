package main

import (
	"log"
	"net/http"
	"os"
	"strings"
	"text/template"
)

var tpl *template.Template
var resText string

func init() {
	tpl = template.Must(template.ParseGlob("templates/*.gohtml"))
}

func main() {
	http.HandleFunc("/", index)
	http.HandleFunc("/ascii-art", process)
	http.HandleFunc("/readme", readme)
	http.Handle("/favicon.ico", http.NotFoundHandler())
	http.Handle("/js/", http.StripPrefix("/js/", http.FileServer(http.Dir("js"))))
	http.Handle("/css/", http.StripPrefix("/css/", http.FileServer(http.Dir("css"))))
	http.ListenAndServe(":8080", nil)
}

func readme(w http.ResponseWriter, req *http.Request) {
	err := tpl.ExecuteTemplate(w, "readme.gohtml", nil)

	if err != nil {
		log.Println(err)
		http.Error(w, "Internal server error", http.StatusInternalServerError)
		return
	}
}

func index(w http.ResponseWriter, req *http.Request) {
	if checkURL(req.URL.Path) {
		err := tpl.ExecuteTemplate(w, "index.gohtml", resText)
		resText = ""

		if err != nil {
			log.Println(err)
			http.Error(w, "Internal server error", http.StatusInternalServerError)
			return
		}
	} else {
		log.Println("404 page not found:", req.URL.Path)
		err := tpl.ExecuteTemplate(w, "404.gohtml", nil)

		if err != nil {
			log.Println(err)
			http.Error(w, "Internal server error", http.StatusInternalServerError)
			return
		}
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
	case "/ascii-art":
	case "/":
	case "/process":
	default:
		if _, err := os.Stat(urlGiven); os.IsNotExist(err) {
			res = false
		}
	}

	return res
}

func process(w http.ResponseWriter, req *http.Request) {

	sendTo := "index.gohtml"

	if req.Method == http.MethodPost {
		subVal := req.FormValue("subVal")

		switch subVal {
		case "subAsciiArt":
			subAsciiArtProc(w, req)
		default:
			log.Println(subVal)

			err := tpl.ExecuteTemplate(w, sendTo, nil)

			if err != nil {
				log.Println(err)
				http.Error(w, "Internal server error", http.StatusInternalServerError)
				return
			}
		}
	}
}

func subAsciiArtProc(w http.ResponseWriter, req *http.Request) {
	textStyleToConvert := req.FormValue("textStyleToConvert")
	textToConvert := req.FormValue("textToConvert")

	if textToConvert != "" && textStyleToConvert != "" {
		log.Println("selected", textStyleToConvert)
		log.Println("Text to convert\n" + textToConvert)
		resText = asciiArt(textToConvert, textStyleToConvert)
	} else {
		log.Println("Nothing selected")
	}

	http.Redirect(w, req, "/", 307)
}

func asciiArt(givenString string, typeSelected string) string {
	res := ""             // Resulting string to output
	letters := []string{} // Array where all letters are stored

	letter := "" // String where to store the current letter

	wordToPrint := strings.Split(givenString, "\n") // The word to print
	typeToPrint := strings.ToLower(typeSelected)    // In what style to print the result in

	shouldContinue := true

	if shouldContinue {
		switch typeToPrint {
		case "thinkertoy":
			filename := "ascii-art/thinkertoy.txt"
			file, err := os.ReadFile(filename)
			if err != nil {
				log.Println(string("[ERROR] File is missing: " + err.Error()))
				shouldContinue = false
				break
			}

			fileContents := string(file)
			for _, char := range fileContents {
				if char != '\r' {
					if char == '\n' {
						char = ' '
					}
					letter += string(char)
				} else {
					letter += " "
					letters = append(letters, letter)
					letter = ""
				}
			}
		case "standard":
			filename := "ascii-art/standard.txt"
			file, err := os.ReadFile(filename)
			if err != nil {
				log.Println(string("[ERROR] File is missing: " + err.Error()))
				shouldContinue = false
				break
			}

			fileContents := string(file)
			for _, char := range fileContents {
				if char != '\n' {
					letter += string(char)
				} else {
					letters = append(letters, letter)
					letter = ""
				}
			}
		case "shadow":
			filename := "ascii-art/shadow.txt"
			file, err := os.ReadFile(filename)
			if err != nil {
				log.Println(string("[ERROR] File is missing: " + err.Error()))
				shouldContinue = false
				break
			}

			fileContents := string(file)
			for _, char := range fileContents {
				if char != '\n' {
					letter += string(char)
				} else {
					letter += " "
					letters = append(letters, letter)
					letter = ""
				}
			}
		default:
			log.Println(string("[ERROR] Select box was tampered with"))
			shouldContinue = false
		}
	}
	if shouldContinue {
		for _, words := range wordToPrint { // Get all words
			for i := 1; i <= 8; i++ { // Go through the rows
				tempRes := ""
				for k, char := range words { // Split words into runes
					if int(char)-31 > 0 && int(char) <= 127 { // If the rune is not out of the given character spectrum
						if (((int(char) - 32) * 9) + i) > 0 { // If the character number is bigger than 0
							tempRes += letters[(((int(char) - 32) * 9) + i)]
						}
					}
					if k == len(words)-1 {
						if tempRes != "" {
							res += tempRes + "</br>"
						}
					}
				}
			}
		}

	}

	if shouldContinue {
		return res
	} else {
		return ""
	}
}
