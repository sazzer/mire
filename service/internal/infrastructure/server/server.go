package server

import (
	"fmt"
	"os"

	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	loglevels "github.com/labstack/gommon/log"
	"github.com/rs/zerolog/log"
	"github.com/ziflex/lecho/v2"
)

// Server represents the actual web server that does the work.
type Server struct {
	server *echo.Echo
}

// New creates a new web server.
func New(config []Configurer) Server {
	logger := lecho.New(
		os.Stderr,
		lecho.WithLevel(loglevels.DEBUG),
		lecho.WithTimestamp(),
	)

	e := echo.New()
	e.Logger = logger

	e.Use(middleware.Recover())
	e.Use(middleware.Gzip())
	e.Use(middleware.CORSWithConfig(middleware.CORSConfig{
		AllowCredentials: true,
	}))
	e.Use(middleware.RequestID())
	e.Use(lecho.Middleware(lecho.Config{
		Logger: logger,
	}))

	for _, c := range config {
		err := c.RegisterRoutes(e)
		if err != nil {
			log.Fatal().Err(err).Msg("Failed to register routes")
		}
	}

	return Server{
		server: e,
	}
}

// Start will start the service listening on the given port.
func (s Server) Start(port uint16) {
	log.Info().Uint16("port", port).Msg("Starting server")

	err := s.server.Start(fmt.Sprintf(":%d", port))
	if err != nil {
		log.Fatal().Err(err).Msg("Failed to start server")
	}
}
