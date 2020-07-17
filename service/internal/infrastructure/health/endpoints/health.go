package endpoints

import (
	"net/http"

	"github.com/sazzer/mire/service/internal/httpresponse"
	"github.com/sazzer/mire/service/internal/infrastructure/health"
)

// Health wraps the various components needed for Healthcheck endpoints.
type Health struct {
	useCase health.HealthcheckUseCase
}

// New creates a new instance of the Health endpoints.
func New(useCase health.HealthcheckUseCase) Health {
	return Health{useCase: useCase}
}

// GetHealth will get the health of the system.
func (h *Health) GetHealth(w http.ResponseWriter, r *http.Request) {
	result := h.useCase.CheckHealth(r.Context())

	components := map[string]ComponentHealthModel{}

	for name, status := range result.Components() {
		if status.Status() == health.StatusHealthy {
			components[name] = ComponentHealthModel{
				Status:  StatusHealthy,
				Message: "",
			}
		} else {
			components[name] = ComponentHealthModel{
				Status:  StatusUnhealthy,
				Message: status.Error().Error(),
			}
		}
	}

	statusCode := http.StatusOK
	systemStatus := StatusHealthy

	if result.Status() == health.StatusUnhealthy {
		statusCode = http.StatusServiceUnavailable
		systemStatus = StatusUnhealthy
	}

	output := SystemHealthModel{
		Status:     systemStatus,
		Components: components,
	}

	httpresponse.RenderJSON(w, statusCode, output)
}
