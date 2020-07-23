package players

import "context"

// Use case for creating a new player.
type Creator interface {
	// Create will create a new player with the given data.
	Create(ctx context.Context, data PlayerData) (Player, error)
}
