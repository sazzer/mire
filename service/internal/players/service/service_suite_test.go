package service_test

import (
	"testing"

	"github.com/sazzer/mire/service/internal/players/db"
	"github.com/sazzer/mire/service/internal/players/service"
	"github.com/sazzer/mire/service/test/databasesuite"
)

type PlayerServiceSuite struct {
	databasesuite.Suite
	service service.PlayerService
}

func NewSuite(t *testing.T) PlayerServiceSuite {
	suite := databasesuite.NewSuite(t)

	repo := db.New(suite.Database)
	service := service.New(repo)

	return PlayerServiceSuite{
		suite,
		service,
	}
}
