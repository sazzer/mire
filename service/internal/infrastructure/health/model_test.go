package health_test

import (
	"errors"
	"testing"

	"github.com/sazzer/mire/service/internal/infrastructure/health"
	"github.com/stretchr/testify/assert"
)

func TestHealthyComponentHealth(t *testing.T) {
	component := health.ComponentHealth{nil}
	assert.Nil(t, component.Error)
	assert.Equal(t, health.StatusHealthy, component.Status())
}

func TestUnhealthyComponentHealth(t *testing.T) {
	component := health.ComponentHealth{errors.New("Oops")}
	assert.Equal(t, errors.New("Oops"), component.Error)
	assert.Equal(t, health.StatusUnhealthy, component.Status())
}

func TestEmptySystemHealth(t *testing.T) {
	system := health.SystemHealth{map[string]health.ComponentHealth{}}
	assert.Equal(t, health.StatusHealthy, system.Status())
	assert.Equal(t, map[string]health.ComponentHealth{}, system.Components)
}

func TestHealthySystemHealth(t *testing.T) {
	system := health.SystemHealth{map[string]health.ComponentHealth{
		"healthy": {nil},
	}}
	assert.Equal(t, health.StatusHealthy, system.Status())
	assert.Equal(t, map[string]health.ComponentHealth{
		"healthy": {nil},
	}, system.Components)
}

func TestUnhealthySystemHealth(t *testing.T) {
	system := health.SystemHealth{map[string]health.ComponentHealth{
		"unhealthy": {errors.New("Oops")},
	}}
	assert.Equal(t, health.StatusUnhealthy, system.Status())
	assert.Equal(t, map[string]health.ComponentHealth{
		"unhealthy": {errors.New("Oops")},
	}, system.Components)
}

func TestMixedSystemHealth(t *testing.T) {
	system := health.SystemHealth{map[string]health.ComponentHealth{
		"healthy":   {nil},
		"unhealthy": {errors.New("Oops")},
	}}
	assert.Equal(t, health.StatusUnhealthy, system.Status())
	assert.Equal(t, map[string]health.ComponentHealth{
		"healthy":   {nil},
		"unhealthy": {errors.New("Oops")},
	}, system.Components)
}
