package router

import (
	"fmt"
	"log"
	"net/http"
	"strings"
)

// ApplyMiddleware wraps a handler with any number of middleware. The last one is the outermost.
func ApplyMiddleware(final http.Handler, funcs ...func(http.Handler) http.Handler) http.Handler {
	for _, fn := range funcs {
		final = fn(final)
	}
	return final
}

// Recover takes a handler (next) and if it panics, it lets another handler handle the request instead.
func Recover(next, recoverer http.Handler, logErr bool) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		defer func() {
			if err := recover(); err != nil {
				if logErr {
					log.Println(err)
					// debug.PrintStack()
				}
				recoverer.ServeHTTP(w, r)
			}
		}()

		next.ServeHTTP(w, r)
	})
}

// Recover500 sends a status 500 error when the handler panics.
func Recover500(next http.Handler) http.Handler {
	return Recover(
		next,
		http.HandlerFunc(func(w http.ResponseWriter, _ *http.Request) {
			code := http.StatusInternalServerError
			http.Error(w, fmt.Sprintf(`%v %v`, code, http.StatusText(code)), code)
		}),
		true,
	)
}

// RedirectTrailingSlash redirects requests with a trailing slash to one without.
func RedirectTrailingSlash(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		path := r.URL.Path
		if path != "/" && strings.HasSuffix(path, "/") {
			http.Redirect(w, r, path[:len(path)-1], http.StatusMovedPermanently)
			return
		}

		next.ServeHTTP(w, r)
	})
}

// LogRequests will log any incoming requests.
func LogRequests(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		client := r.Header.Get("X-Forwarded-For")
		if client == "" {
			client = r.RemoteAddr
		}
		log.Printf("%4s request for %v from %v\n", r.Method, r.URL.RequestURI(), client)
		next.ServeHTTP(w, r)
	})
}
