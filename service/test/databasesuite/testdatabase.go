package databasesuite

import (
	"context"
	"fmt"
	"testing"

	"github.com/rs/zerolog/log"
	"github.com/stretchr/testify/assert"
	"github.com/testcontainers/testcontainers-go"
	"github.com/testcontainers/testcontainers-go/wait"
)

type TestDatabase struct {
	container testcontainers.Container
}

func NewDatabase(t *testing.T) TestDatabase {
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

	return TestDatabase{postgres}
}

func (d TestDatabase) Close(t *testing.T) {
	log.Info().Msg("Closing database")

	ctx := context.Background()
	err := d.container.Terminate(ctx)
	assert.NoError(t, err)
}

func (d TestDatabase) URL(t *testing.T) string {
	ctx := context.Background()
	ip, err := d.container.Host(ctx)
	assert.NoError(t, err)

	port, err := d.container.MappedPort(ctx, "5432")
	assert.NoError(t, err)

	return fmt.Sprintf("postgres://postgres@%s:%s", ip, port.Port())
}
