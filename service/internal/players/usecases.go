package players

import "context"

// Use case for finding a player by their authentication details.
type AuthentictionDetailsFinder interface {
	// Find a player by the authentication System ID and Player ID.
	FindByAuthenticationDetails(ctx context.Context, systemID string, playerID string) (Player, error)
}

// Use case for creating a new player.
type Creator interface {
	// Create a player with the given data
	CreatePlayer(ctx context.Context, data PlayerData) (Player, error)
}
