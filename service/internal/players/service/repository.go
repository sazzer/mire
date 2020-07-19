package service

import (
	"context"

	"github.com/sazzer/mire/service/internal/players"
)

// PlayerRepository is a repository for accessing player data in the database.
type PlayerRepository interface {
	CreatePlayer(ctx context.Context, data players.PlayerData) (players.Player, error)
}
