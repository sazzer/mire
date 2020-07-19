package service_test

import (
	"context"

	"github.com/sazzer/mire/service/internal/players"
)

func (suite *PlayerServiceSuite) TestCreatePlayerSuccess() {
	data := players.PlayerData{
		Email:           "test@example.com",
		DisplayName:     "Test User",
		Authentications: []players.Authentication{},
	}

	player, err := suite.service.CreatePlayer(context.Background(), data)
	suite.NoError(err)

	suite.Equal(data, player.Data)
}
