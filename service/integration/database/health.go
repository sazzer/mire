package database

import "context"

// Healthcheck will check the health of the component and return an error if the component is unhealthy.
func (d Database) Healthcheck(ctx context.Context) error {
	return d.db.PingContext(ctx)
}
