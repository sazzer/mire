package internal

import (
	"github.com/sazzer/mire/service/internal/infrastructure/health"
	"github.com/sazzer/mire/service/internal/infrastructure/server"
)

// Service represents the actual Mire service and everything within it
type Service struct {
	server server.Server
}

// New creates a new instance of the service
func New() Service {
	health := health.New(map[string]health.Healthchecker{})

	return Service{
		server: server.New([]server.Configurer{
			health,
		}),
	}
}

// Start will start the service listening on the given port
func (s Service) Start(port uint16) {
	s.server.Start(port)
}
