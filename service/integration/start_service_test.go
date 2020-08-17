package integration_test

import (
	"testing"

	"github.com/sazzer/mire/service/integration"
)

func TestStartService(t *testing.T) {
	service := integration.NewService(t)
	defer service.Close()
}