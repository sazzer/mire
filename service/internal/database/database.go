package database

import (

	// Drivers for the Postgres database connection.
	_ "github.com/jackc/pgx/stdlib"

	"context"
	"database/sql"
	"errors"

	"github.com/jmoiron/sqlx"
	"github.com/rs/zerolog/log"
)

// ErrNoRows indicates that we queried for a single row and didn't get any back.
var ErrNoRows = errors.New("no row found")

// ErrUnexpected indicates an unexpected error happened when calling the database.
var ErrUnexpected = errors.New("unexpected error calling database")

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

// Query the database for a single object.
func (d *Database) QueryOne(ctx context.Context, dest interface{}, query string, args ...interface{}) error {
	log.Debug().Str("sql", query).Msg("Executing SQL")

	if err := d.db.GetContext(ctx, dest, query, args...); err != nil {
		log.Warn().Err(err).Str("sql", query).Msg("Error executing SQL")

		if errors.Is(err, sql.ErrNoRows) {
			return ErrNoRows
		}

		return ErrUnexpected
	}

	log.Debug().Str("sql", query).Interface("result", dest).Msg("Executed SQL")

	return nil
}
