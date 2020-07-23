package health

import "context"

// PENDING: Refactor this to a flatter structure.

// Healthchecker represents any component that can report on its own health.
type Healthchecker interface {
	// Healthcheck will check the health of the component and return an error if the component is unhealthy
	Healthcheck(ctx context.Context) error
}
