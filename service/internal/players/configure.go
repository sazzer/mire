package players

import (
	"github.com/go-chi/chi"
	"github.com/sazzer/mire/service/internal/infrastructure/database"
)

// Config represents the configuration for the players subsystem.
type Config struct {
	Service service
}

// New creates a new instance of the configuration.
func New(database database.Database) Config {
	repository := repository{database}
	service := service{repository}

	return Config{service}
}

// RegisterRoutes will register the HTTP routes for the healthchecks.
func (c Config) RegisterRoutes(r chi.Router) {
}
