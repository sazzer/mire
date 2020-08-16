package integration

import (
	"testing"

	"github.com/sazzer/mire/service/internal"
)

// Wrapper around the actual service for integration test purposes.
type Service struct {
	t       *testing.T
	service internal.Service
}

// Create a new instance of the test service.
func NewService(t *testing.T) Service {
	service := internal.NewService()

	return Service{
		t:       t,
		service: service,
	}
}

// Ensure that the service is shut down correctly.
func (s *Service) Close() {

}
