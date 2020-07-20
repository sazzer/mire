package service_test

import (
	"context"
	"testing"

	"github.com/sazzer/mire/service/internal/players"
	"github.com/stretchr/testify/assert"
)

func TestCreatePlayerSuccess(t *testing.T) {
	suite := NewSuite(t)
	defer suite.Close(t)

	data := players.PlayerData{
		Email:           "test@example.com",
		DisplayName:     "Test User",
		Authentications: []players.Authentication{},
	}

	player, err := suite.service.CreatePlayer(context.Background(), data)
	assert.NoError(t, err)

	assert.Equal(t, data, player.Data)
}
