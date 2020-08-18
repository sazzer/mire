package authorization

import (
	"time"

	"github.com/benbjohnson/clock"
)

// Component represents the authorization component.
type Component struct {
	Service service
}

// NewComponent creates a new instance of the authorization component.
func NewComponent(clock clock.Clock, duration time.Duration, signingKey string) Component {
	return Component{
		Service: newService(clock, duration, signingKey),
	}
}
