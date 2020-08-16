package database_test

import (
	"testing"

	"github.com/sazzer/mire/service/internal/database"
	"github.com/sazzer/mire/service/tests"
)

func TestMigrateDatabase(t *testing.T) {
	wrapper := tests.NewDatabase(t)
	defer wrapper.Close(t)

	db := database.NewDatabase(wrapper.URL(t))

	database.Migrate(db)
}

func TestMigrateDatabaseTwice(t *testing.T) {
	wrapper := tests.NewDatabase(t)
	defer wrapper.Close(t)

	db := database.NewDatabase(wrapper.URL(t))

	database.Migrate(db)
	database.Migrate(db)
}
