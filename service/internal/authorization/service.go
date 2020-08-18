package authorization

import (
	"time"

	"github.com/benbjohnson/clock"
)

// Service layer for working with security contexts.
type service struct {
	clock      clock.Clock
	duration   time.Duration
	signingKey []byte
}

// Create a new instance of the service layer.
func newService(clock clock.Clock, duration time.Duration, signingKey string) service {
	return service{
		clock:      clock,
		duration:   duration,
		signingKey: []byte(signingKey),
	}
}
