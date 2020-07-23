package health

import (
	"context"

	"github.com/rs/zerolog/log"
)

// Service is the standard implementation of the healthchecker.
type Service struct {
	Components map[string]Healthchecker
}

// CheckHealth will check the overall system health and return this.
func (s Service) CheckHealth(ctx context.Context) SystemHealth {
	components := map[string]ComponentHealth{}
	for name, component := range s.Components {
		componentHealth := component.Healthcheck(ctx)
		log.Debug().
			AnErr("err", componentHealth).
			Bool("healthy", componentHealth == nil).
			Str("component", name).
			Msg("Health of component")

		components[name] = ComponentHealth{componentHealth}
	}

	return SystemHealth{components}
}
