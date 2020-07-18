package internal

import (
	"github.com/sazzer/mire/service/internal/infrastructure/database"
	"github.com/sazzer/mire/service/internal/infrastructure/health"
	healthConfig "github.com/sazzer/mire/service/internal/infrastructure/health/configure"
	"github.com/sazzer/mire/service/internal/infrastructure/server"
)

// Service represents the actual Mire service and everything within it.
type Service struct {
	server server.Server
}

// New creates a new instance of the service.
func New(databaseURL string) Service {
	database := database.New(databaseURL)
	database.Migrate()

	health := healthConfig.New(map[string]health.Healthchecker{
		"db": database,
	})

	return Service{
		server: server.New([]server.Configurer{
			health,
		}),
	}
}

// Start will start the service listening on the given port.
func (s Service) Start(port uint16) {
	s.server.Start(port)
}
