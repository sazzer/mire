package health

import "github.com/sazzer/mire/service/internal/infrastructure/server"

// Config represents the configuration of the healthcheck component.
type Config struct {
	service Service
}

// NewConfig constructs a new configuration for the healthcheck component.
func NewConfig(components map[string]Healthchecker) Config {
	return Config{
		service: Service{components},
	}
}

// ServerConfig builds the server configuration needed to register the healthcheck endpoints
// with the HTTP Server.
func (c *Config) ServerConfig() server.Configurer {
	return endpoints{c.service}
}
