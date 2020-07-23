package players_test

import (
	"context"
	"testing"

	"github.com/sazzer/mire/service/internal/players"
	"github.com/sazzer/mire/service/test/databasesuite"
	"github.com/stretchr/testify/suite"
)

// PENDING: Consider only starting the container once and sharing it between tests
// Needs the ability to reset the database between tests

type serviceSuite struct {
	suite.Suite
	db      databasesuite.Suite
	service players.Service
}

func (suite *serviceSuite) SetupTest() {
	suite.db = databasesuite.NewSuite(suite.T())

	players := players.New(suite.db.Database)
	suite.service = players.Service
}

func (suite *serviceSuite) TearDownTest() {
	suite.db.Close(suite.T())
}

func TestService(t *testing.T) {
	suite.Run(t, new(serviceSuite))
}

func (suite *serviceSuite) TestCreateSimplePlayer() {
	player, err := suite.service.Create(context.Background(), players.PlayerData{
		Email:           "testuser@example.com",
		DisplayName:     "Test User",
		Authentications: []players.Authentication{},
	})

	suite.NoError(err)
	suite.Equal("testuser@example.com", player.Data.Email)
	suite.Equal("Test User", player.Data.DisplayName)
	suite.Equal([]players.Authentication{}, player.Data.Authentications)
}
