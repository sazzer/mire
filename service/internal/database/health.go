package database

import "context"

// Check the health of the database connection.
func (d Database) CheckHealth(ctx context.Context) error {
	return d.db.PingContext(ctx)
}
