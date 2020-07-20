package databasesuite

import (
	"testing"

	"github.com/sazzer/mire/service/internal/infrastructure/database"
)

type Suite struct {
	db       TestDatabase
	Database database.Database
}

func NewSuite(t *testing.T) Suite {
	dbContainer := NewDatabase(t)

	database := database.New(dbContainer.URL(t))
	database.Migrate()

	return Suite{
		db:       dbContainer,
		Database: database,
	}
}

func (suite *Suite) Close(t *testing.T) {
	suite.db.Close(t)
}
