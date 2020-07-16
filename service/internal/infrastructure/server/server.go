package server

import (
	"fmt"

	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	"github.com/rs/zerolog/log"
)

// Server represents the actual web server that does the work
type Server struct {
	server *echo.Echo
}

// New creates a new web server
func New() Server {
	e := echo.New()

	e.Use(middleware.Logger())
	e.Use(middleware.Recover())
	e.Use(middleware.Gzip())
	e.Use(middleware.CORSWithConfig(middleware.CORSConfig{
		AllowCredentials: true,
	}))
	e.Use(middleware.RequestID())

	return Server{
		server: e,
	}
}

// Start will start the service listening on the given port
func (s Server) Start(port uint16) {
	log.Info().Uint16("port", port).Msg("Starting server")

	err := s.server.Start(fmt.Sprintf(":%d", port))
	if err != nil {
		log.Fatal().Err(err).Msg("Failed to start server")
	}
}
