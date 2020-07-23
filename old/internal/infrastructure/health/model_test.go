package health_test

import (
	"errors"
	"testing"

	"github.com/sazzer/mire/service/internal/infrastructure/health"
	"github.com/stretchr/testify/assert"
)

func TestHealthyComponent(t *testing.T) {
	component := health.NewComponentHealth(nil)
	assert.Nil(t, component.Error())
	assert.Equal(t, health.StatusHealthy, component.Status())
}

func TestUnhealthyComponent(t *testing.T) {
	component := health.NewComponentHealth(errors.New("Oops"))
	assert.Equal(t, errors.New("Oops"), component.Error())
	assert.Equal(t, health.StatusUnhealthy, component.Status())
}

func TestEmptySystem(t *testing.T) {
	system := health.NewSystemHealth(map[string]health.ComponentHealth{})
	assert.Equal(t, health.StatusHealthy, system.Status())
	assert.Equal(t, map[string]health.ComponentHealth{}, system.Components())
}

func TestHealthySystem(t *testing.T) {
	system := health.NewSystemHealth(map[string]health.ComponentHealth{
		"healthy": health.NewComponentHealth(nil),
	})
	assert.Equal(t, health.StatusHealthy, system.Status())
	assert.Equal(t, map[string]health.ComponentHealth{
		"healthy": health.NewComponentHealth(nil),
	}, system.Components())
}

func TestUnhealthySystem(t *testing.T) {
	system := health.NewSystemHealth(map[string]health.ComponentHealth{
		"unhealthy": health.NewComponentHealth(errors.New("Oops")),
	})
	assert.Equal(t, health.StatusUnhealthy, system.Status())
	assert.Equal(t, map[string]health.ComponentHealth{
		"unhealthy": health.NewComponentHealth(errors.New("Oops")),
	}, system.Components())
}

func TestMixedSystem(t *testing.T) {
	system := health.NewSystemHealth(map[string]health.ComponentHealth{
		"healthy":   health.NewComponentHealth(nil),
		"unhealthy": health.NewComponentHealth(errors.New("Oops")),
	})
	assert.Equal(t, health.StatusUnhealthy, system.Status())
	assert.Equal(t, map[string]health.ComponentHealth{
		"healthy":   health.NewComponentHealth(nil),
		"unhealthy": health.NewComponentHealth(errors.New("Oops")),
	}, system.Components())
}
