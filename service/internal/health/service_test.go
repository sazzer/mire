package health_test

import (
	"errors"
	"testing"

	"github.com/sazzer/mire/service/internal/health"
	"github.com/stretchr/testify/assert"
)

func TestNoComponents(t *testing.T) {
	service := health.NewService()

	health := service.CheckHealth()

	assert.True(t, health.Healthy())
	assert.Equal(t, 0, len(health.Components))
}

func TestPassingComponent(t *testing.T) {
	service := health.NewService()
	service.WithComponent("passing", health.HealthcheckerFunc(func() error { return nil }))

	health := service.CheckHealth()

	assert.True(t, health.Healthy())
	assert.Equal(t, 1, len(health.Components))

	assert.True(t, health.Components["passing"].Healthy())
	assert.Nil(t, health.Components["passing"].Error)
}

func TestFailingComponent(t *testing.T) {
	service := health.NewService()
	service.WithComponent("failing", health.HealthcheckerFunc(func() error { return errors.New("Oops") }))

	health := service.CheckHealth()

	assert.False(t, health.Healthy())
	assert.Equal(t, 1, len(health.Components))

	assert.False(t, health.Components["failing"].Healthy())
	assert.EqualError(t, health.Components["failing"].Error, "Oops")
}

func TestMixedComponent(t *testing.T) {
	service := health.NewService()
	service.WithComponent("passing", health.HealthcheckerFunc(func() error { return nil }))
	service.WithComponent("failing", health.HealthcheckerFunc(func() error { return errors.New("Oops") }))

	health := service.CheckHealth()

	assert.False(t, health.Healthy())
	assert.Equal(t, 2, len(health.Components))

	assert.True(t, health.Components["passing"].Healthy())
	assert.Nil(t, health.Components["passing"].Error)

	assert.False(t, health.Components["failing"].Healthy())
	assert.EqualError(t, health.Components["failing"].Error, "Oops")
}
