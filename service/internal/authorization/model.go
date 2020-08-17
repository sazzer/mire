package authorization

import (
	"time"
)

// The unique ID of a Security Context.
type SecurityContextID string

func (s SecurityContextID) string() string {
	return string(s)
}

// The unique ID of the Principal that a Security Context represents.
type PrincipalID string

func (s PrincipalID) string() string {
	return string(s)
}

// The actual Security Context that is authorized to perform some actions.
type SecurityContext struct {
	ID        SecurityContextID
	Principal PrincipalID
	Issued    time.Time
	Expires   time.Time
}

// A signed security context.
type SignedSecurityContext string
