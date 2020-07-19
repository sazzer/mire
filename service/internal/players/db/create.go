package db

import (
	"context"

	"github.com/rs/zerolog/log"
	"github.com/sazzer/mire/service/internal/model"
	"github.com/sazzer/mire/service/internal/players"
)

func (r Repository) CreatePlayer(ctx context.Context, data players.PlayerData) (players.Player, error) {
	identity, err := model.NewIdentity()
	if err != nil {
		log.Warn().Err(err).Msg("Failed to create identity for new player")
		return players.Player{}, err
	}

	output := PlayerModel{}

	err = r.database.QueryOne(ctx, &output,
		`INSERT INTO players(player_id, version, created, updated, email, display_name, authentication)
		VALUES(:player_id, :version, :created, :updated, :email, :display_name, :authentication)
		RETURNING *`,
		NewModelFromData(identity, data),
	)
	if err != nil {
		log.Warn().Err(err).Interface("player", data).Msg("Failed to insert player into database")
		return players.Player{}, err
	}

	return output.ToModel(), nil
}
