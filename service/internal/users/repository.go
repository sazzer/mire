package users

import (
	"context"
	"time"

	"github.com/rs/zerolog/log"
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

type userRecord struct {
	ID              UserID    `db:"user_id"`
	Version         string    `db:"version"`
	Created         time.Time `db:"created"`
	Updated         time.Time `db:"updated"`
	Email           Email     `db:"email"`
	DisplayName     string    `db:"display_name"`
	Authentications string    `db:"authentications"`
}

func (u userRecord) UserModel() UserModel {
	return UserModel{
		ID:      u.ID,
		Version: u.Version,
		Created: u.Created,
		Updated: u.Updated,
		UserData: UserData{
			Email:           u.Email,
			DisplayName:     u.DisplayName,
			Authentications: []Authentication{},
		},
	}
}

// Get the single user that has the provided unique ID.
func (r repository) GetUserByID(ctx context.Context, userID UserID) UserModel {
	var user userRecord

	if err := r.db.QueryOne(ctx, &user, "SELECT * FROM users WHERE user_id = $1", &userID); err != nil {
		log.Info().Err(err).Interface("user_id", userID).Msg("Failed to find user")
	}

	return user.UserModel()
}
