package database

import "context"

// Healthcheck will check the health of the database and return an error it is unhealthy.
func (d Database) Healthcheck(ctx context.Context) error {
	return d.db.DB.PingContext(ctx)
}
