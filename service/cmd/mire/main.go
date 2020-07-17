package main

import (
	"os"
	"time"

	_ "github.com/joho/godotenv/autoload"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
	"github.com/sazzer/mire/service/internal"
)

func main() {
	config := LoadConfig()

	if config.Debug {
		log.Logger = log.Output(zerolog.ConsoleWriter{Out: os.Stderr, TimeFormat: time.RFC3339})
	}

	log.Logger = log.With().Caller().Logger()

	log.Info().Msg("Starting Mire...")

	service := internal.New()
	service.Start(config.Port)
}
