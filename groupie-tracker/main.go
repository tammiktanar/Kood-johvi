package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"os"
	"strconv"
	"strings"
	"text/template"
)

var tpl *template.Template

type ArtistInfo []struct {
	ID           int      `json:"id"`
	Image        string   `json:"image"`
	Name         string   `json:"name"`
	Members      []string `json:"members"`
	CreationDate int      `json:"creationDate"`
	FirstAlbum   string   `json:"firstAlbum"`
}

type ArtistInfoByID struct {
	ID           int      `json:"id"`
	Image        string   `json:"image"`
	Name         string   `json:"name"`
	Members      []string `json:"members"`
	CreationDate int      `json:"creationDate"`
	FirstAlbum   string   `json:"firstAlbum"`
}

type DateInfoByID struct {
	ID    int      `json:"id"`
	Dates []string `json:"dates"`
}

type LocationInfoByID struct {
	ID        int      `json:"id"`
	Locations []string `json:"locations"`
}

type RelationsInfoByID struct {
	ID             int    `json:"id"`
	DatesLocations string `json:"datesLocations"`
}

type ArtistPage struct {
	ArtistInfo        ArtistInfoByID
	DateInfo          DateInfoByID
	LocationInfo      LocationInfoByID
	ArtistInfoToPrint string
}

type Relation struct {
	Index []struct {
		ID             int                 `json:"id"`
		DatesLocations map[string][]string `json:"datesLocations"`
	} `json:"index`
}

func init() {
	tpl = template.Must(template.ParseGlob("templates/*.gohtml"))
}

func main() {
	http.HandleFunc("/", index)
	http.HandleFunc("/process", process)
	http.HandleFunc("/readme", readme)
	http.HandleFunc("/artist/", artistPage)
	http.Handle("/favicon.ico", http.NotFoundHandler())
	http.Handle("/js/", http.StripPrefix("/js/", http.FileServer(http.Dir("js"))))
	http.Handle("/css/", http.StripPrefix("/css/", http.FileServer(http.Dir("css"))))
	http.ListenAndServe(":8080", nil)
}

func artistPage(w http.ResponseWriter, req *http.Request) {

	nr, err := strconv.Atoi(strings.Replace(req.URL.Path, "/artist/", "", -1))

	if err != nil {
		log.Println(err)
		http.Error(w, "Internal server error", http.StatusInternalServerError)
		return
	}

	check, groupieTrackerIndex := fillArtistInfoByID(nr)
	if !check {
		log.Println("Data filling failed")
		http.Error(w, "Internal server error", http.StatusInternalServerError)
		return
	}

	err = tpl.ExecuteTemplate(w, "artistPage.gohtml", groupieTrackerIndex)

	if err != nil {
		log.Println(err)
		http.Error(w, "Internal server error", http.StatusInternalServerError)
		return
	}
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
		check, groupieTrackerIndex := fillArtistInfo()
		if !check {
			log.Println("Data filling failed")
			http.Error(w, "Internal server error", http.StatusInternalServerError)
			return
		}

		err := tpl.ExecuteTemplate(w, "index.gohtml", groupieTrackerIndex)

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
	} else {
		log.Println("Nothing selected")
	}

	http.Redirect(w, req, "/", 307)
}

func fillArtistInfo() (bool, ArtistInfo) {
	var res ArtistInfo
	resp, err := http.Get("https://groupietrackers.herokuapp.com/api")
	if err != nil {
		log.Println(err)
		return false, res
	} else {
		if resp != nil {
			_, err := ioutil.ReadAll(resp.Body)
			if err != nil {
				log.Println(err)
				return false, res
			}
		} else {
			log.Println("Empty")
			return false, res
		}
	}

	resp, err = http.Get("https://groupietrackers.herokuapp.com/api/artists")
	if err != nil {
		log.Println(err)
		return false, res
	} else {
		if resp != nil {
			body, err := ioutil.ReadAll(resp.Body)
			if err != nil {
				log.Println(err)
				return false, res
			} else {
				err := json.Unmarshal(body, &res)
				if err != nil {
					fmt.Println(err)
					return false, res
				}
			}
		} else {
			log.Println("Empty")
			return false, res
		}
	}

	return true, res
}

func fillArtistInfoByID(id int) (bool, ArtistPage) {

	var res ArtistPage
	resp, err := http.Get("https://groupietrackers.herokuapp.com/api")
	if err != nil {
		log.Println(err)
		return false, res
	} else {
		if resp != nil {
			_, err := ioutil.ReadAll(resp.Body)
			if err != nil {
				log.Println(err)
				return false, res
			}
		} else {
			log.Println("Empty")
			return false, res
		}
	}

	resp, err = http.Get("https://groupietrackers.herokuapp.com/api/artists/" + strconv.Itoa(id))
	if err != nil {
		log.Println(err)
		return false, res
	} else {
		if resp != nil {
			body, err := ioutil.ReadAll(resp.Body)
			if err != nil {
				log.Println(err)
				return false, res
			} else {

				err := json.Unmarshal(body, &res.ArtistInfo)
				if err != nil {
					fmt.Println(err)
					return false, res
				}
			}
		} else {
			log.Println("Empty")
			return false, res
		}
	}

	resp, err = http.Get("https://groupietrackers.herokuapp.com/api/locations/" + strconv.Itoa(id))
	if err != nil {
		log.Println(err)
		return false, res
	} else {
		if resp != nil {
			body, err := ioutil.ReadAll(resp.Body)
			if err != nil {
				log.Println(err)
				return false, res
			} else {
				err := json.Unmarshal(body, &res.LocationInfo)
				if err != nil {
					fmt.Println(err)
					return false, res
				}
			}
		} else {
			log.Println("Empty")
			return false, res
		}
	}

	resp, err = http.Get("https://groupietrackers.herokuapp.com/api/dates/" + strconv.Itoa(id))
	if err != nil {
		log.Println(err)
		return false, res
	} else {
		if resp != nil {
			body, err := ioutil.ReadAll(resp.Body)
			if err != nil {
				log.Println(err)
				return false, res
			} else {
				err := json.Unmarshal(body, &res.DateInfo)
				if err != nil {
					fmt.Println(err)
					return false, res
				}
			}
		} else {
			log.Println("Empty")
			return false, res
		}
	}

	i := -1
	for k, date := range res.DateInfo.Dates {
		if strings.Contains(date, "*") {
			if k != 0 {
				res.ArtistInfoToPrint += `
				</div>
				</div>
				<hr>`
			}
			i++
			locationForPrint := strings.Title(strings.Replace(strings.Replace(res.LocationInfo.Locations[i], "-", ", ", -1), "_", " ", -1))
			res.ArtistInfoToPrint += `<div class="row">
			<div class="col-8">
				<label style="font-weight:bold;">` + locationForPrint + `</label>
				</div>
				<div class="col-4">`
		}
		res.ArtistInfoToPrint += strings.Replace(strings.Replace(date, "*", "", -1), "-", "/", -1) + "</br>"

	}
	res.ArtistInfoToPrint += `
	</div>
	</div>
	<hr>`

	return true, res
}
