package db

import (
	"time"

	"github.com/google/uuid"
	"github.com/sazzer/mire/service/internal/model"
	"github.com/sazzer/mire/service/internal/players"
)

type PlayerModel struct {
	PlayerID uuid.UUID `db:"player_id"`
	Version  uuid.UUID `db:"version"`
	Created  time.Time `db:"created"`
	Updated  time.Time `db:"updated"`

	Email          string `db:"email"`
	DisplayName    string `db:"display_name"`
	Authentication string `db:"authentication"`
}

func NewModelFromData(identity model.Identity, data players.PlayerData) PlayerModel {
	return PlayerModel{
		PlayerID:       identity.ID,
		Version:        identity.Version,
		Created:        identity.Created,
		Updated:        identity.Updated,
		Email:          data.Email,
		DisplayName:    data.DisplayName,
		Authentication: "{}",
	}
}

func (p PlayerModel) ToModel() players.Player {
	return players.Player{
		Identity: model.Identity{
			ID:      p.PlayerID,
			Version: p.Version,
			Created: p.Created,
			Updated: p.Updated,
		},
		Data: players.PlayerData{
			Email:           p.Email,
			DisplayName:     p.DisplayName,
			Authentications: []players.Authentication{},
		},
	}
}
