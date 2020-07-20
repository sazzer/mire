package service_test

import (
	"testing"

	"github.com/sazzer/mire/service/internal/infrastructure/database"
	"github.com/sazzer/mire/service/internal/players/db"
	"github.com/sazzer/mire/service/internal/players/service"
	"github.com/sazzer/mire/service/tests"
)

type PlayerServiceSuite struct {
	db       tests.Database
	database database.Database
	service  service.PlayerService
}

func NewSuite(t *testing.T) PlayerServiceSuite {
	dbContainer := tests.NewDatabase(t)

	database := database.New(dbContainer.URL(t))
	database.Migrate()

	repo := db.New(database)
	service := service.New(repo)

	return PlayerServiceSuite{
		db:       dbContainer,
		database: database,
		service:  service,
	}
}

func (suite *PlayerServiceSuite) Close(t *testing.T) {
	suite.db.Close(t)
}
