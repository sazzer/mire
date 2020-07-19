package configure

import (
	"github.com/go-chi/chi"
	"github.com/sazzer/mire/service/internal/infrastructure/database"
	"github.com/sazzer/mire/service/internal/players/db"
	"github.com/sazzer/mire/service/internal/players/service"
)

// Config represents the configuration for the players subsystem.
type Config struct {
	Service service.PlayerService
}

// New creates a new instance of the configuration.
func New(database database.Database) Config {
	repository := db.New(database)
	service := service.New(repository)

	return Config{service}
}

// RegisterRoutes will register the HTTP routes for the healthchecks.
func (c Config) RegisterRoutes(r chi.Router) {
}
