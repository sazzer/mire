package main

import (
	"github.com/kelseyhightower/envconfig"
	"github.com/rs/zerolog/log"
)

// Config represents the configuration of the app.
type Config struct {
	Debug       bool
	Port        uint16 `default:"8000"`
	DatabaseURL string `required:"true" envconfig:"DATABASE_URL"`
}

// LoadConfig will load the application configuration from the environment.
func LoadConfig() Config {
	var config Config
	err := envconfig.Process("", &config)

	if err != nil {
		log.Fatal().Err(err).Msg("Failed to load configuration")
	}

	return config
}
