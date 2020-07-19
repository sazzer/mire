package model

import (
	"time"

	"github.com/google/uuid"
)

// Identity represents the identity of some model resource.
type Identity struct {
	ID      uuid.UUID
	Version uuid.UUID
	Created time.Time
	Updated time.Time
}
