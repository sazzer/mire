package server

import "github.com/rs/zerolog/log"

// Server represents the actual HTTP Server
type Server struct{}

// NewServer creates a new HTTP Server
func NewServer() Server {
	return Server{}
}

// Start will start the HTTP server listening for connection
func (s *Server) Start() {
	log.Info().Msg("Starting HTTP Server...")
}
