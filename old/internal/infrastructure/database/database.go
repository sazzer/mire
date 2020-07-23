package database

import (
	// The database drivers
	_ "github.com/jackc/pgx/stdlib"

	"github.com/jmoiron/sqlx"
	"github.com/rs/zerolog/log"
)

// Database represents a connection to the database.
type Database struct {
	db *sqlx.DB
}

// New creates a new database connection pool.
func New(url string) Database {
	db, err := sqlx.Connect("pgx", url)
	if err != nil {
		log.Fatal().Err(err).Str("url", url).Msg("Failed to connect to database")
	}

	log.Info().Str("url", url).Msg("Connected to database")

	return Database{db}
}
