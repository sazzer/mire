package health

import (
	"context"

	"github.com/rs/zerolog/log"
)

// The actual service layer for performing healthchecks.
type Service struct {
	components map[string]Healthchecker
}

// Create a new instance of the healthcheck service.
func NewService(components map[string]Healthchecker) Service {
	return Service{
		components: components,
	}
}

// Representation of the health of a single component.
type ComponentHealth struct {
	Error error
}

// Check if this component is healthy.
func (c ComponentHealth) Healthy() bool {
	return c.Error == nil
}

// Representation of the health of the entire system.
type SystemHealth struct {
	Components map[string]ComponentHealth
}

// Check if the entire service is healthy.
func (s SystemHealth) Healthy() bool {
	for _, component := range s.Components {
		if !component.Healthy() {
			return false
		}
	}

	return true
}

// Actually check the health of the service.
func (s Service) CheckHealth(ctx context.Context) SystemHealth {
	result := map[string]ComponentHealth{}

	for name, component := range s.components {
		e := component.CheckHealth(ctx)
		log.Info().Str("name", name).AnErr("result", e).Msg("Component health")
		result[name] = ComponentHealth{e}
	}

	return SystemHealth{result}
}
