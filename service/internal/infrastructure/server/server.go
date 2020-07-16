package server

import "github.com/rs/zerolog/log"

// Server represents the actual web server that does the work
type Server struct {
}

// New creates a new web server
func New() Server {
	return Server{}
}

// Start will start the service listening on the given port
func (s Server) Start(port uint16) {
	log.Info().Uint16("port", port).Msg("Starting server")
}
