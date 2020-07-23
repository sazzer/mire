package health

import (
	"net/http"

	"github.com/go-chi/chi"
)

type endpoints struct{}

func (e *endpoints) getHealth(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(400)
}

func (e endpoints) RegisterRoutes(r chi.Router) {
	r.Get("/health", e.getHealth)
}
