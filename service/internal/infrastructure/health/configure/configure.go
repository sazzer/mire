package configure

import (
	"github.com/labstack/echo/v4"
	"github.com/sazzer/mire/service/internal/infrastructure/health"
	"github.com/sazzer/mire/service/internal/infrastructure/health/endpoints"
	"github.com/sazzer/mire/service/internal/infrastructure/health/service"
)

// Config represents the configuration for the healthchecks subsystem
type Config struct {
	components map[string]health.Healthchecker
}

// New creates a new instance of the configuration
func New(components map[string]health.Healthchecker) Config {
	return Config{components: components}
}

// RegisterRoutes will register the HTTP routes for the healthchecks
func (c Config) RegisterRoutes(e *echo.Echo) error {
	healthcheckService := service.New(c.components)
	endpoints := endpoints.New(healthcheckService)

	e.GET("/health", endpoints.GetHealth)
	return nil
}
