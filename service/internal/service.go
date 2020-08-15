package internal

import (
	"github.com/rs/zerolog/log"
	"github.com/sazzer/mire/service/internal/health"
	"github.com/sazzer/mire/service/internal/server"
)

// Service represents the actual Mire service.
type Service struct {
	server server.Server
}

// NewService creates a new instance of the Mire service.
func NewService() Service {
	log.Debug().Msg("Building Mire...")

	health := health.NewComponent(map[string]health.Healthchecker{})

	server := server.NewServer(health)

	return Service{
		server: server,
	}
}

// Start will start the Mire service running and ready to accept connections.
func (s *Service) Start(port uint16) {
	log.Info().Msg("Starting Mire...")
	s.server.Start(port)
}
