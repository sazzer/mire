package database

import (
	"context"
	"errors"

	"github.com/jmoiron/sqlx"
	"github.com/rs/zerolog/log"
)

var (
	ErrNoRows       = errors.New("mire: db: no rows returned")
	ErrMultipleRows = errors.New("mire: db: multiple errors returned")
)

type UnexpectedError struct {
	cause error
}

func (e UnexpectedError) Error() string {
	return "mire: db: unexpected database error"
}

func (e UnexpectedError) Unwrap() error {
	return e.cause
}

func (d Database) queryOne(ctx context.Context, output interface{}, f func(context.Context) (*sqlx.Rows, error)) error {
	rows, err := f(ctx)
	if err != nil {
		log.Warn().Err(err).Msg("Failed to execute query")
		return UnexpectedError{err}
	}

	// Ensure the resultset is closed afterwards
	defer func() {
		err := rows.Close()
		if err != nil {
			log.Error().Err(err).Msg("Failed to close resultset")
		}
	}()

	// Fail on zero rows
	if !rows.Next() {
		log.Warn().Err(rows.Err()).Msg("No rows returned")
		return ErrNoRows
	}

	// Parse the first row into our output object
	err = rows.StructScan(output)
	if err != nil {
		log.Warn().Err(err).Msg("Failed to parse result")
		return UnexpectedError{err}
	}

	log.Debug().Interface("result", output).Msg("Read value from database")

	// Failure on 2+ rows
	if rows.Next() {
		return ErrMultipleRows
	}

	return nil
}

func (d Database) QueryOneNamed(ctx context.Context, output interface{}, query string, binds interface{}) error {
	log.Debug().Str("sql", query).Interface("binds", binds).Msg("Executing query")

	return d.queryOne(ctx, output, func(ctx context.Context) (*sqlx.Rows, error) {
		return d.db.NamedQueryContext(ctx, query, binds)
	})
}

func (d Database) QueryOneOrdered(ctx context.Context, output interface{}, query string, binds ...interface{}) error {
	log.Debug().Str("sql", query).Interface("binds", binds).Msg("Executing query")

	return d.queryOne(ctx, output, func(ctx context.Context) (*sqlx.Rows, error) {
		return d.db.QueryxContext(ctx, query, binds...)
	})
}
