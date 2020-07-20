package service_test

import (
	"testing"

	"github.com/sazzer/mire/service/internal/infrastructure/database"
	"github.com/sazzer/mire/service/internal/players/db"
	"github.com/sazzer/mire/service/internal/players/service"
)

type PlayerServiceSuite struct {
	database.Suite
	service service.PlayerService
}

func NewSuite(t *testing.T) PlayerServiceSuite {
	suite := database.NewSuite(t)

	repo := db.New(suite.Database)
	service := service.New(repo)

	return PlayerServiceSuite{
		suite,
		service,
	}
}
