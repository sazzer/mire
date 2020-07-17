package service

import (
	"github.com/rs/zerolog/log"
	"github.com/sazzer/mire/service/internal/infrastructure/health"
)

// Service is the standard implementation of the healthchecker
type Service struct {
	components map[string]health.Healthchecker
}

// New creates a new new healthcheck service
func New(components map[string]health.Healthchecker) Service {
	return Service{components: components}
}

// CheckHealth will check the overall system health and return this
func (s Service) CheckHealth() health.SystemHealth {
	components := map[string]health.ComponentHealth{}
	for name, component := range s.components {
		componentHealth := component.Healthcheck()
		log.Debug().
			AnErr("status", componentHealth).
			Bool("healthy", componentHealth == nil).
			Str("component", name).
			Msg("Health of component")
		components[name] = health.NewComponentHealth(componentHealth)
	}
	return health.NewSystemHealth(components)
}
