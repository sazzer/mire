package players

import (
	"context"

	"github.com/sazzer/mire/service/internal/infrastructure/database"
)

// repository represents the database repository for storing player data.
type repository struct {
	database database.Database
}

// Create will create a new player with the given data.
func (r *repository) Create(ctx context.Context, data PlayerData) (Player, error) {
	panic("Not implemented yet")
}
