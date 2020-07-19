package service

import (
	"context"

	"github.com/rs/zerolog/log"
	"github.com/sazzer/mire/service/internal/players"
)

// Create a player with the given data.
func (s PlayerService) CreatePlayer(ctx context.Context, data players.PlayerData) (players.Player, error) {
	player, err := s.repository.CreatePlayer(ctx, data)
	if err != nil {
		log.Warn().Err(err).Interface("player", data).Msg("Failed to create player")
	}

	return player, err
}
