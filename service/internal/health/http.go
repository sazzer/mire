package health

import (
	"net/http"

	"github.com/go-chi/render"
)

// Http represents all of the HTTP Handlers for the healthchecks.
type HTTP struct{}

// Gets the actual health of the system.
func (http *HTTP) getHealth(w http.ResponseWriter, r *http.Request) {
	render.JSON(w, r, "Hello")
}
