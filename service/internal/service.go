package internal

import (
	"net/http"

	"github.com/rs/zerolog/log"
	"github.com/sazzer/mire/service/internal/database"
	"github.com/sazzer/mire/service/internal/health"
	"github.com/sazzer/mire/service/internal/server"
)

// Service represents the actual Mire service.
type Service struct {
	server server.Server
}

// NewService creates a new instance of the Mire service.
func NewService(databaseURL string) Service {
	log.Debug().Msg("Building Mire...")

	db := database.NewDatabase(databaseURL)
	database.Migrate(db)

	health := health.NewComponent(map[string]health.Healthchecker{
		"database": db,
	})

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

// Inject a request into the HTTP Server and handle the response.
func (s *Service) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	s.server.ServeHTTP(w, r)
}
