package authorization

import (
	"time"
)

// The unique ID of a Security Context.
type SecurityContextID string

// The unique ID of the Principal that a Security Context represents.
type PrincipalID string

// The actual Security Context that is authorized to perform some actions.
type SecurityContext struct {
	ID        SecurityContextID
	Principal PrincipalID
	Issued    time.Time
	Expires   time.Time
}

// A signed security context.
type SignedSecurityContext string
