package service_test

import (
	"testing"

	"github.com/sazzer/mire/service/internal/infrastructure/database"
	"github.com/sazzer/mire/service/internal/players/db"
	"github.com/sazzer/mire/service/internal/players/service"
	"github.com/sazzer/mire/service/tests"
	"github.com/stretchr/testify/suite"
)

type PlayerServiceSuite struct {
	suite.Suite
	db       tests.Database
	database database.Database
	service  service.PlayerService
}

func (suite *PlayerServiceSuite) SetupTest() {
	suite.db = tests.NewDatabase(suite.T())

	suite.database = database.New(suite.db.URL(suite.T()))
	suite.database.Migrate()

	repo := db.New(suite.database)
	suite.service = service.New(repo)
}

func (suite *PlayerServiceSuite) TearDownTest() {
	suite.db.Close(suite.T())
}

func TestPlayerServiceSuite(t *testing.T) {
	suite.Run(t, new(PlayerServiceSuite))
}
