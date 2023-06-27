package api

import (
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"path"
	"social-network/models"
	"social-network/router"
)

const maxUploadSize = 1024 * 1024 // 1MB

func FileUpload(w http.ResponseWriter, r *http.Request) {
	r.Body = http.MaxBytesReader(w, r.Body, maxUploadSize)
	if err := r.ParseMultipartForm(maxUploadSize); err != nil {
		log.Println("The uploaded file was too big.")
		http.Error(w, "The uploaded file is too big. Please choose an file that's less than 1MB in size", http.StatusBadRequest)
		return
	}

	// The argument to FormFile must match the name attribute of the file input on the frontend
	file, fileHeader, err := r.FormFile("file")
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	defer file.Close()

	token, err := Database.File.Insert(file, fileHeader.Filename)
	if err != nil {
		panic(err)
		return
	}

	response := struct {
		Token string `json:"token"`
	}{
		token,
	}
	writeJSON(w, response)
}

func FileDownload(w http.ResponseWriter, r *http.Request) {
	token := router.GetSlug(r, 0)

	fileData, err := Database.File.Get(token)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	file, err := os.Open(path.Join(models.UploadsPath, fileData.Token+fileData.Extension))
	if err != nil {
		panic(err)
	}

	var contentType string

	switch fileData.Extension {
	case ".png":
		contentType = "image/png"
	case ".jpg":
	case ".jpeg":
		contentType = "image/jpeg"
	default:
		contentType = "application/octet-stream"
	}

	w.Header().Set("Content-Type", contentType)
	w.Header().Set("Content-Disposition", fmt.Sprintf(`inline; filename="%s"`, fileData.Name))

	_, err = io.Copy(w, file)
	if err != nil {
		panic(err)
	}
}
