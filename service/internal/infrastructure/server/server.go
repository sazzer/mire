package server

import (
	"fmt"
	"net/http"

	"github.com/go-chi/chi"
	"github.com/go-chi/chi/middleware"
	"github.com/go-chi/cors"
	"github.com/rs/zerolog/log"
)

// Server represents the actual web server that does the work.
type Server struct {
	router *chi.Mux
}

// New creates a new web server.
func New(config []Configurer) Server {
	r := chi.NewRouter()

	r.Use(middleware.RequestID)
	r.Use(middleware.RealIP)
	r.Use(middleware.Logger)
	r.Use(middleware.Recoverer)
	r.Use(middleware.GetHead)
	r.Use(middleware.Compress(5))
	r.Use(cors.Handler(cors.Options{
		AllowedMethods:   []string{"GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"},
		ExposedHeaders:   []string{"Link"},
		AllowCredentials: true,
		MaxAge:           300,
	}))

	for _, c := range config {
		err := c.RegisterRoutes(r)
		if err != nil {
			log.Fatal().Err(err).Msg("Failed to register routes")
		}
	}

	return Server{
		router: r,
	}
}

// Start will start the service listening on the given port.
func (s Server) Start(port uint16) {
	log.Info().Uint16("port", port).Msg("Starting server")

	err := http.ListenAndServe(fmt.Sprintf(":%d", port), s.router)
	if err != nil {
		log.Fatal().Err(err).Msg("Failed to start server")
	}
}
