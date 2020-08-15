package main

import (
	"github.com/rs/zerolog/log"

	"github.com/kelseyhightower/envconfig"
)

// Config represents the configuration of the app.
type Config struct {
	Debug bool
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
