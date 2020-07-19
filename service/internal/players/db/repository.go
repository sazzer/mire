package db

import "github.com/sazzer/mire/service/internal/infrastructure/database"

// DatabaseRepository is a Player Repository implementation backed by the Database.
type Repository struct {
	database database.Database
}

// New creates a new database repository for working with players.
func New(database database.Database) Repository {
	return Repository{database}
}
