package internal

import "github.com/sazzer/mire/service/internal/infrastructure/server"

// Service represents the actual Mire service and everything within it
type Service struct {
	server server.Server
}

// New creates a new instance of the service
func New() Service {
	return Service{
		server: server.New(),
	}
}

// Start will start the service listening on the given port
func (s Service) Start(port uint16) {
	s.server.Start(port)
}
