package health

import "context"

// Healthchecker represents any component that can report on its own health.
type Healthchecker interface {
	// Healthcheck will check the health of the component and return an error if the component is unhealthy.
	Healthcheck(ctx context.Context) error
}

// HealthcheckerFunc is a function type that implements the Healthchecker interface.
type HealthcheckerFunc func(context.Context) error

// Healthcheck will check the health of the component and return an error if the component is unhealthy.
func (f HealthcheckerFunc) Healthcheck(ctx context.Context) error {
	return f(ctx)
}
