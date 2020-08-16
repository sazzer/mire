package health_test

import (
	"context"
	"errors"
	"testing"

	"github.com/sazzer/mire/service/internal/health"
	"github.com/stretchr/testify/assert"
)

func TestNoComponents(t *testing.T) {
	service := health.NewService(map[string]health.Healthchecker{})

	health := service.CheckHealth(context.Background())

	assert.True(t, health.Healthy())
	assert.Equal(t, 0, len(health.Components))
}

func TestPassingComponent(t *testing.T) {
	service := health.NewService(map[string]health.Healthchecker{
		"passing": health.HealthcheckerFunc(func(ctx context.Context) error { return nil }),
	})

	health := service.CheckHealth(context.Background())

	assert.True(t, health.Healthy())
	assert.Equal(t, 1, len(health.Components))

	assert.True(t, health.Components["passing"].Healthy())
	assert.Nil(t, health.Components["passing"].Error)
}

func TestFailingComponent(t *testing.T) {
	service := health.NewService(map[string]health.Healthchecker{
		"failing": health.HealthcheckerFunc(func(ctx context.Context) error { return errors.New("Oops") }),
	})

	health := service.CheckHealth(context.Background())

	assert.False(t, health.Healthy())
	assert.Equal(t, 1, len(health.Components))

	assert.False(t, health.Components["failing"].Healthy())
	assert.EqualError(t, health.Components["failing"].Error, "Oops")
}

func TestMixedComponent(t *testing.T) {
	service := health.NewService(map[string]health.Healthchecker{
		"passing": health.HealthcheckerFunc(func(ctx context.Context) error { return nil }),
		"failing": health.HealthcheckerFunc(func(ctx context.Context) error { return errors.New("Oops") }),
	})

	health := service.CheckHealth(context.Background())

	assert.False(t, health.Healthy())
	assert.Equal(t, 2, len(health.Components))

	assert.True(t, health.Components["passing"].Healthy())
	assert.Nil(t, health.Components["passing"].Error)

	assert.False(t, health.Components["failing"].Healthy())
	assert.EqualError(t, health.Components["failing"].Error, "Oops")
}
