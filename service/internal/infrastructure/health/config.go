package health

import "github.com/sazzer/mire/service/internal/infrastructure/server"

type Config struct{}

func (c *Config) ServerConfig() server.Configurer {
	return endpoints{}
}
