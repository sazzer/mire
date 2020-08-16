package health

import (
	"net/http"

	"github.com/go-chi/render"
)

// Http represents all of the HTTP Handlers for the healthchecks.
type HTTP struct {
	service Service
}

// JSON representation of the health of a single component.
type componentHealthModel struct {
	Healthy bool   `json:"healthy"`
	Message string `json:"message,omitempty"`
}

// JSON representation of the health of the entire system.
type systemHealthModel struct {
	Healthy    bool                            `json:"healthy"`
	Components map[string]componentHealthModel `json:"components"`
}

// Gets the actual health of the system.
func (h *HTTP) getHealth(w http.ResponseWriter, r *http.Request) {
	systemHealth := h.service.CheckHealth(r.Context())

	result := systemHealthModel{
		Healthy:    systemHealth.Healthy(),
		Components: map[string]componentHealthModel{},
	}

	for name, component := range systemHealth.Components {
		if component.Healthy() {
			result.Components[name] = componentHealthModel{
				Healthy: true,
			}
		} else {
			result.Components[name] = componentHealthModel{
				Healthy: false,
				Message: component.Error.Error(),
			}
		}
	}

	if result.Healthy {
		render.Status(r, http.StatusOK)
	} else {
		render.Status(r, http.StatusServiceUnavailable)
	}

	render.JSON(w, r, result)
}
