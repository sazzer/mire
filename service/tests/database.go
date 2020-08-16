package tests

import (
	"context"
	"fmt"
	"testing"

	"github.com/rs/zerolog/log"
	"github.com/stretchr/testify/assert"
	"github.com/testcontainers/testcontainers-go"
	"github.com/testcontainers/testcontainers-go/wait"
)

// Wrapper around a Postgres database for the test service.
type Database struct {
	container testcontainers.Container
}

// Create a new database wrapper.
func NewDatabase(t *testing.T) Database {
	if testing.Short() {
		t.Skip("Skipping database tests")

		return Database{}
	}

	log.Info().Msg("Starting database")

	ctx := context.Background()
	req := testcontainers.ContainerRequest{
		Image:        "postgres:11.6-alpine",
		ExposedPorts: []string{"5432/tcp"},
		WaitingFor:   wait.ForListeningPort("5432/tcp"),
	}

	postgres, err := testcontainers.GenericContainer(ctx, testcontainers.GenericContainerRequest{
		ContainerRequest: req,
		Started:          true,
	})
	assert.NoError(t, err)

	return Database{
		container: postgres,
	}
}

// Close the database down.
func (d *Database) Close(t *testing.T) {
	log.Info().Msg("Stopping database")

	ctx := context.Background()
	err := d.container.Terminate(ctx)
	assert.NoError(t, err)
}

// Get the connection string for connecting to the test database.
func (d *Database) URL(t *testing.T) string {
	ctx := context.Background()
	ip, err := d.container.Host(ctx)
	assert.NoError(t, err)

	port, err := d.container.MappedPort(ctx, "5432")
	assert.NoError(t, err)

	return fmt.Sprintf("postgres://postgres@%s:%s", ip, port.Port())
}
