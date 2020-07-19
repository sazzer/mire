package model

import (
	"time"

	"github.com/google/uuid"
	"github.com/rs/zerolog/log"
)

// Identity represents the identity of some model resource.
type Identity struct {
	ID      uuid.UUID
	Version uuid.UUID
	Created time.Time
	Updated time.Time
}

// Create a new Identity object for a new model instance.
func NewIdentity() (Identity, error) {
	id, err := uuid.NewRandom()
	if err != nil {
		log.Warn().Err(err).Msg("Failed to generate ID")
		return Identity{}, err
	}

	version, err := uuid.NewRandom()
	if err != nil {
		log.Warn().Err(err).Msg("Failed to generate Version")
		return Identity{}, err
	}

	return Identity{
		ID:      id,
		Version: version,
		Created: time.Now(),
		Updated: time.Now(),
	}, nil
}
