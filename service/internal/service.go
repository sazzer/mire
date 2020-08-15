package internal

import "github.com/rs/zerolog/log"

// Service represents the actual Mire service
type Service struct{}

// NewService creates a new instance of the Mire service
func NewService() Service {
	log.Debug().Msg("Building Mire...")

	return Service{}
}

// Start will start the Mire service running and ready to accept connections
func (s *Service) Start() {
	log.Info().Msg("Starting Mire...")
}
