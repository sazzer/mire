package endpoints

import (
	"net/http"

	"github.com/labstack/echo/v4"
	"github.com/sazzer/mire/service/internal/infrastructure/health"
)

// Health wraps the various components needed for Healthcheck endpoints
type Health struct {
	useCase health.HealthcheckUseCase
}

// New creates a new instance of the Health endpoints
func New(useCase health.HealthcheckUseCase) Health {
	return Health{useCase: useCase}
}

// GetHealth will get the health of the system
func (h *Health) GetHealth(c echo.Context) error {
	return c.String(http.StatusOK, "Hello, World!")
}
