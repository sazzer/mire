package internal

import (
	"github.com/sazzer/mire/service/integration/database"
	"github.com/sazzer/mire/service/internal/infrastructure/health"
	"github.com/sazzer/mire/service/internal/infrastructure/server"
)

// Mire represents the actual service
type Mire struct {
	server server.Server
}

// New creates a new instance of the Mire service
func New(databaseURL string) Mire {
	db := database.New(databaseURL)

	health := health.NewConfig(
		map[string]health.Healthchecker{
			"db": db,
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
