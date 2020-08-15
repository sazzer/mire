package health

import "github.com/rs/zerolog/log"

// The actual service layer for performing healthchecks.
type Service struct {
	components map[string]Healthchecker
}

// Create a new instance of the healthcheck service.
func NewService() Service {
	return Service{
		components: map[string]Healthchecker{},
	}
}

// Add a component to the healthcheck service
func (s Service) WithComponent(name string, component Healthchecker) {
	s.components[name] = component
}

// Representation of the health of a single component.
type ComponentHealth struct {
	Error error
}

// Check if this component is healthy.
func (c ComponentHealth) Healthy() bool {
	return c.Error == nil
}

// Representation of the health of the entire service.
type ServiceHealth struct {
	Components map[string]ComponentHealth
}

// Check if the entire service is healthy.
func (s ServiceHealth) Healthy() bool {
	for _, component := range s.Components {
		if !component.Healthy() {
			return false
		}
	}

	return true
}

// Actually check the health of the service.
func (s Service) CheckHealth() ServiceHealth {
	result := map[string]ComponentHealth{}

	for name, component := range s.components {
		e := component.CheckHealth()
		log.Info().Str("name", name).AnErr("result", e).Msg("Component health")
		result[name] = ComponentHealth{e}
	}

	return ServiceHealth{result}
}
