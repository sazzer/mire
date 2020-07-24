package database

import (
	// The database drivers
	_ "github.com/jackc/pgx/stdlib"

	"github.com/jmoiron/sqlx"
	"github.com/rs/zerolog/log"
)

// Database represents a connection to the Postgres database.
type Database struct {
	db *sqlx.DB
}

// New creates a new connection to the Postgres database.
func New(url string) Database {
	log.Debug().Str("url", url).Msg("Connecting to database")
	db, err := sqlx.Connect("pgx", url)
	if err != nil {
		log.Fatal().Err(err).Str("url", url).Msg("Failed to connect to database")
	}

	return Database{db}
}
