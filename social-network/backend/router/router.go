package router

import (
	"context"
	"fmt"
	"net/http"
	"regexp"
	"strings"
)

// Router is used to configure routes. Use method name functions like Router.Get to configure routes.
// The route chosen will be the first one that matches.
type Router struct {
	routes []route
}

type route struct {
	method  string
	regex   *regexp.Regexp
	handler http.Handler
}

// New creates a new router.
func New() Router {
	return Router{}
}

// Part 1: Defining routes

// Get makes a route for a GET request.
// The pattern is processed as regex and any capture groups are accessible with GetSlug in the handler.
func (router *Router) Get(pattern string, handler http.HandlerFunc) {
	router.addRoute("GET", pattern, handler)
}

// Post makes a route for a POST request.
// The pattern is processed as regex and any capture groups are accessible with GetSlug in the handler.
func (router *Router) Post(pattern string, handler http.HandlerFunc) {
	router.addRoute("POST", pattern, handler)
}

func (router *Router) addRoute(method string, pattern string, handler http.Handler) {
	rt := route{
		method:  method,
		regex:   regexp.MustCompile(fmt.Sprintf(`^%v$`, pattern)),
		handler: handler,
	}

	router.routes = append(router.routes, rt)
}

// Part 2: Serving routes

func (router Router) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	var allowed []string

	// Attempt to find a matching route
	for _, rt := range router.routes {
		match := rt.regex.FindStringSubmatch(r.URL.Path)

		if match == nil {
			continue
		}

		if rt.method != r.Method {
			// Wrong method
			allowed = append(allowed, rt.method)
			continue
		}

		if len(match) > 1 {
			ctx := r.Context()
			ctx = context.WithValue(ctx, "routerSlugs", match[1:])
			r = r.WithContext(ctx)
		}

		rt.handler.ServeHTTP(w, r)
		return
	}

	// No matches
	if len(allowed) > 0 {
		// Had a match, but wrong method
		w.Header().Set("Allow", strings.Join(allowed, ", "))
		code := http.StatusMethodNotAllowed
		http.Error(w, fmt.Sprintf(`%v %v`, code, http.StatusText(code)), code)
		return
	}

	http.NotFound(w, r)
}

// GetSlug retrieves a capture group that was saved during route matching (0-indexed).
func GetSlug(r *http.Request, index int) string {
	slugs, ok := r.Context().Value("routerSlugs").([]string)
	if !ok || index >= len(slugs) {
		return ""
	}
	return slugs[index]
}
