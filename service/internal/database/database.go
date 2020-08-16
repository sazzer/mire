package database

import (

	// Drivers for the Postgres database connection.
	_ "github.com/jackc/pgx/stdlib"
	"github.com/jmoiron/sqlx"
	"github.com/rs/zerolog/log"
)

// Wrapper around the database connection.
type Database struct {
	db *sqlx.DB
}

// Create a new database connection.
func NewDatabase(url string) Database {
	log.Info().Str("url", url).Msg("Connecting to database")

	db, err := sqlx.Connect("pgx", url)
	if err != nil {
		log.Fatal().Err(err).Msg("Failed to connect to database")
	}

	return Database{
		db: db,
	}
}
