package health

import (
	"net/http"

	"github.com/labstack/echo/v4"
)

// Config represents the configuration for the healthchecks subsystem
type Config struct {
}

// New creates a new instance of the configuration
func New() Config {
	return Config{}
}

// RegisterRoutes will register the HTTP routes for the healthchecks
func (c Config) RegisterRoutes(e *echo.Echo) error {
	e.GET("/health", func(c echo.Context) error {
		return c.String(http.StatusOK, "Hello, World!")
	})
	return nil
}
