package users

import (
	"github.com/sazzer/mire/service/internal/database"
)

// The users repository, for working with user records in the data store.
type repository struct {
	db database.Database
}

// Create a new users repository.
func newRepository(db database.Database) repository {
	return repository{
		db: db,
	}
}
