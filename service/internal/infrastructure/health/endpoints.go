package health

import (
	"net/http"

	"github.com/go-chi/chi"
	"github.com/sazzer/mire/service/internal/utils/httpresponse"
)

// componentHealthModel represents the HTTP Model for the health of a single component.
type componentHealthModel struct {
	Status  string `json:"status"`
	Message string `json:"message,omitempty"`
}

// systemHealthModel represents the HTTP Model for the health of the entire system.
type systemHealthModel struct {
	Status     string                          `json:"status"`
	Components map[string]componentHealthModel `json:"components"`
}

// endpoints is a struct that contains all of the HTTP Endpoints for checking the health of the system.
type endpoints struct {
	service Service
}

// getHealth is the endpoint for getting the health of the system.
func (e *endpoints) getHealth(w http.ResponseWriter, r *http.Request) {
	systemHealth := e.service.CheckHealth(r.Context())

	var response systemHealthModel
	response.Components = map[string]componentHealthModel{}

	for componentName, componentHealth := range systemHealth.Components {
		if componentHealth.Error == nil {
			response.Components[componentName] = componentHealthModel{
				Status: "HEALTHY",
			}
		} else {
			response.Components[componentName] = componentHealthModel{
				Status:  "UNHEALTHY",
				Message: componentHealth.Error.Error(),
			}
		}
	}

	response.Status = "HEALTHY"
	statusCode := http.StatusOK
	if systemHealth.Status() == StatusUnhealthy {
		response.Status = "UNHEALTHY"
		statusCode = http.StatusServiceUnavailable
	}

	httpresponse.RenderJSON(w, statusCode, response)
}

// RegisterRoutes is the implementation of the server.Configurer interface to register these
// endpoints with the HTTP Server.
func (e endpoints) RegisterRoutes(r chi.Router) {
	r.Get("/health", e.getHealth)
}
