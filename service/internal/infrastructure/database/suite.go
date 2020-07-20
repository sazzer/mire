package database

import (
	"testing"

	"github.com/sazzer/mire/service/test/databasesuite"
)

type Suite struct {
	db       databasesuite.TestDatabase
	Database Database
}

func NewSuite(t *testing.T) Suite {
	dbContainer := databasesuite.NewDatabase(t)

	database := New(dbContainer.URL(t))
	database.Migrate()

	return Suite{
		db:       dbContainer,
		Database: database,
	}
}

func (suite *Suite) Close(t *testing.T) {
	suite.db.Close(t)
}
