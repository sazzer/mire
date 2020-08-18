package authorization

import (
	"time"

	"github.com/benbjohnson/clock"
)

// Service layer for working with security contexts.
type Service struct {
	clock      clock.Clock
	duration   time.Duration
	signingKey []byte
}

// Create a new instance of the service layer.
func NewService(clock clock.Clock, duration time.Duration, signingKey string) Service {
	return Service{
		clock:      clock,
		duration:   duration,
		signingKey: []byte(signingKey),
	}
}
