package health_test

import (
	"context"
	"errors"
	"testing"

	"github.com/sazzer/mire/service/internal/infrastructure/health"
	"github.com/stretchr/testify/assert"
)

type MockComponent struct {
	error error
}

func (m MockComponent) Healthcheck(ctx context.Context) error {
	return m.error
}

func TestEmptySystem(t *testing.T) {
	system := health.Service{map[string]health.Healthchecker{}}
	result := system.CheckHealth(context.Background())

	assert.Equal(t, health.StatusHealthy, result.Status())
	assert.Equal(t, map[string]health.ComponentHealth{}, result.Components)
}

func TestHealthySystem(t *testing.T) {
	system := health.Service{map[string]health.Healthchecker{
		"healthy": MockComponent{error: nil},
	}}
	result := system.CheckHealth(context.Background())

	assert.Equal(t, health.StatusHealthy, result.Status())
	assert.Equal(t, map[string]health.ComponentHealth{
		"healthy": health.ComponentHealth{nil},
	}, result.Components)
}

func TestUnhealthySystem(t *testing.T) {
	system := health.Service{map[string]health.Healthchecker{
		"unhealthy": MockComponent{error: errors.New("Oops")},
	}}
	result := system.CheckHealth(context.Background())

	assert.Equal(t, health.StatusUnhealthy, result.Status())
	assert.Equal(t, map[string]health.ComponentHealth{
		"unhealthy": health.ComponentHealth{errors.New("Oops")},
	}, result.Components)
}

func TestMixedSystem(t *testing.T) {
	system := health.Service{map[string]health.Healthchecker{
		"healthy":   MockComponent{error: nil},
		"unhealthy": MockComponent{error: errors.New("Oops")},
	}}
	result := system.CheckHealth(context.Background())

	assert.Equal(t, health.StatusUnhealthy, result.Status())
	assert.Equal(t, map[string]health.ComponentHealth{
		"healthy":   health.ComponentHealth{nil},
		"unhealthy": health.ComponentHealth{errors.New("Oops")},
	}, result.Components)
}
