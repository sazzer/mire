package health

import "context"

// Healthchecker represents any component that can report on it's own health.
type Healthchecker interface {
	// CheckHealth checks the health of the component and returns either nil if the component
	// is healthy or else an error representing the reason the component is unhealthy.
	CheckHealth(ctx context.Context) error
}

// Function type that can be used as a healthchecker.
type HealthcheckerFunc func(ctx context.Context) error

func (f HealthcheckerFunc) CheckHealth(ctx context.Context) error {
	return f(ctx)
}
