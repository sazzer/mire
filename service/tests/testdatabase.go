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

type Database struct {
	container testcontainers.Container
}

func newDatabase(t *testing.T) Database {
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

	return Database{postgres}
}

func (d Database) close(t *testing.T) {
	log.Info().Msg("Closing database")

	ctx := context.Background()
	err := d.container.Terminate(ctx)
	assert.NoError(t, err)
}

func (d Database) url(t *testing.T) string {
	ctx := context.Background()
	ip, err := d.container.Host(ctx)
	assert.NoError(t, err)

	port, err := d.container.MappedPort(ctx, "5432")
	assert.NoError(t, err)

	return fmt.Sprintf("postgres://postgres@%s:%s", ip, port.Port())
}
