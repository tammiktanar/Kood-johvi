package tpl

import (
	"fmt"
	"net/http"
	"text/template"
)

func RenderTemplates(w http.ResponseWriter, execTpl string, data interface{}, parseTpls ...string) {
	tpl, err := template.ParseFiles(parseTpls...)
	if err != nil {
		fmt.Println(err)
		http.Error(w, err.Error(), 500)
		return
	}

	err = tpl.ExecuteTemplate(w, execTpl, data)
	if err != nil {
		fmt.Println(err)
		http.Error(w, err.Error(), 500)
		return
	}

}
