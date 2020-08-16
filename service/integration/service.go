package integration

import (
	"testing"

	"github.com/sazzer/mire/service/internal"
	"github.com/sazzer/mire/service/tests"
)

// Wrapper around the actual service for integration test purposes.
type Service struct {
	t       *testing.T
	service internal.Service
	db      tests.Database
}

// Create a new instance of the test service.
func NewService(t *testing.T) Service {
	db := tests.NewDatabase(t)
	service := internal.NewService(db.URL(t))

	return Service{
		t:       t,
		service: service,
		db:      db,
	}
}

// Ensure that the service is shut down correctly.
func (s *Service) Close() {
	s.db.Close(s.t)
}
