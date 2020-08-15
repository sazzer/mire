package health

import "net/http"

// Http represents all of the HTTP Handlers for the healthchecks.
type HTTP struct{}

// Gets the actual health of the system.
func (http *HTTP) getHealth(w http.ResponseWriter, r *http.Request) {
	_, _ = w.Write([]byte("welcome"))
}
