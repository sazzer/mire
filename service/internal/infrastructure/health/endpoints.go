package health

import (
	"net/http"

	"github.com/go-chi/chi"
	"github.com/sazzer/mire/service/internal/utils/httpresponse"
)

type componentHealthModel struct {
	Status  string `json:"status"`
	Message string `json:"message,omitempty"`
}

type systemHealthModel struct {
	Status     string                          `json:"status"`
	Components map[string]componentHealthModel `json:"components"`
}

type endpoints struct{}

func (e *endpoints) getHealth(w http.ResponseWriter, r *http.Request) {
	response := systemHealthModel{
		Status:     "Healthy",
		Components: map[string]componentHealthModel{},
	}

	httpresponse.RenderJSON(w, 200, response)
}

func (e endpoints) RegisterRoutes(r chi.Router) {
	r.Get("/health", e.getHealth)
}
