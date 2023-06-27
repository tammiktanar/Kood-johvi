package forumEnv

import (
	"html/template"
	"io/fs"
	"os"
	"path"
	"strconv"
	"strings"
	"time"
)

func CreateTemplates(rootPath string) map[string]*template.Template {
	// New file system for globbing
	fileSystem := os.DirFS(rootPath)

	// One path for every template we generate
	templPaths, _ := fs.Glob(fileSystem, "*.html")

	// We will be storing templates here
	templates := make(map[string]*template.Template)

	for _, templPath := range templPaths {
		// start with server/example.html
		base := path.Base(templPath)                     // example.html
		name := strings.TrimSuffix(base, path.Ext(base)) // example

		templates[name] = createTemplate(fileSystem, templPath, name)
	}

	return templates
}

func createTemplate(fileSystem fs.FS, templPath, name string) *template.Template {
	// Add all the HTML files in the specific folder into the slice as patterns
	specificFiles, _ := fs.Glob(fileSystem, path.Join("specific", name, "*.html"))
	patterns := append(
		[]string{templPath, "universal/*.html"},
		specificFiles...,
	)

	// Put all the files that match the patterns into the template
	newTemplate := template.Must(template.New(name).Funcs(getFuncMap()).ParseFS(fileSystem, patterns...))

	return newTemplate
}

// Set custom functions for all templates
func getFuncMap() template.FuncMap {
	return template.FuncMap{
		"itoa": func(num int) string {
			return strconv.Itoa(num)
		},
		"divide": func(a, b int) int {
			return a / b
		},
		"age": func(t time.Time) int {
			_, years, _, _, _, _, _, _ := elapsed(t, time.Now())
			return years
		},
	}
}

// From https://stackoverflow.com/questions/36530251/time-since-with-months-and-years
func daysIn(year int, month time.Month) int {
	return time.Date(year, month, 0, 0, 0, 0, 0, time.UTC).Day()
}

func elapsed(from, to time.Time) (inverted bool, years, months, days, hours, minutes, seconds, nanoseconds int) {
	if from.Location() != to.Location() {
		to = to.In(to.Location())
	}

	inverted = false
	if from.After(to) {
		inverted = true
		from, to = to, from
	}

	y1, M1, d1 := from.Date()
	y2, M2, d2 := to.Date()

	h1, m1, s1 := from.Clock()
	h2, m2, s2 := to.Clock()

	ns1, ns2 := from.Nanosecond(), to.Nanosecond()

	years = y2 - y1
	months = int(M2 - M1)
	days = d2 - d1

	hours = h2 - h1
	minutes = m2 - m1
	seconds = s2 - s1
	nanoseconds = ns2 - ns1

	if nanoseconds < 0 {
		nanoseconds += 1e9
		seconds--
	}
	if seconds < 0 {
		seconds += 60
		minutes--
	}
	if minutes < 0 {
		minutes += 60
		hours--
	}
	if hours < 0 {
		hours += 24
		days--
	}
	if days < 0 {
		days += daysIn(y2, M2-1)
		months--
	}
	if days < 0 {
		days += daysIn(y2, M2)
		months--
	}
	if months < 0 {
		months += 12
		years--
	}
	return
}
