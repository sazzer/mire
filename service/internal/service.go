package internal

import (
	"context"
	"errors"

	"github.com/sazzer/mire/service/internal/infrastructure/health"
	healthConfig "github.com/sazzer/mire/service/internal/infrastructure/health/configure"
	"github.com/sazzer/mire/service/internal/infrastructure/server"
)

// Service represents the actual Mire service and everything within it
type Service struct {
	server server.Server
}

type MockHealthComponent struct {
	error error
}

func (m MockHealthComponent) Healthcheck(ctx context.Context) error {
	return m.error
}

// New creates a new instance of the service
func New() Service {
	health := healthConfig.New(map[string]health.Healthchecker{
		"mock": MockHealthComponent{error: errors.New("Oops")},
	})

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
