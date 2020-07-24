package internal

import (
	"context"
	"errors"

	"github.com/sazzer/mire/service/internal/infrastructure/health"
	"github.com/sazzer/mire/service/internal/infrastructure/server"
)

// Mire represents the actual service
type Mire struct {
	server server.Server
}

// New creates a new instance of the Mire service
func New(databaseURL string) Mire {
	health := health.NewConfig(
		map[string]health.Healthchecker{
			"passing": health.HealthcheckerFunc(func(_ context.Context) error { return nil }),
			"failing": health.HealthcheckerFunc(func(_ context.Context) error { return errors.New("Oops") }),
		},
	)

	return Mire{
		server: server.New([]server.Configurer{
			health.ServerConfig(),
		}),
	}
}

// Start will start the Mire service listening on the given port
func (m *Mire) Start(port uint16) {
	m.server.Start(port)
}
