package authorization

import (
	"time"

	"github.com/google/uuid"

	"github.com/benbjohnson/clock"
)

// Service layer for working with security contexts.
type Service struct {
	clock    clock.Clock
	duration time.Duration
}

// Create a new instance of the service layer.
func NewService(clock clock.Clock, duration time.Duration) Service {
	return Service{
		clock:    clock,
		duration: duration,
	}
}

// Generate a Security Context for the given Principal.
func (s *Service) Generate(principal PrincipalID) SecurityContext {
	id := SecurityContextID(uuid.New().String())
	issued := s.clock.Now().UTC()
	expires := issued.Add(s.duration)

	return SecurityContext{
		ID:        id,
		Issued:    issued,
		Expires:   expires,
		Principal: principal,
	}
}
