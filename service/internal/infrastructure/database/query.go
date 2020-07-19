package database

import (
	"context"
	"errors"

	"github.com/rs/zerolog/log"
)

func (d Database) QueryOne(ctx context.Context, output interface{}, query string, binds interface{}) error {
	log.Debug().Str("sql", query).Interface("binds", binds).Msg("Executing query")

	// Actually do the query
	rows, err := d.db.NamedQueryContext(ctx, query, binds)
	if err != nil {
		log.Warn().Err(err).Str("sql", query).Interface("binds", binds).Msg("Failed to execute query")
		return err
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
		return errors.New("no rows returned")
	}

	// Parse the first row into our output object
	err = rows.StructScan(output)
	if err != nil {
		log.Warn().Err(err).Str("sql", query).Interface("binds", binds).Msg("Failed to parse result")
		return err
	}

	log.Debug().Str("sql", query).Interface("binds", binds).Interface("result", output).Msg("Read value from database")

	// Failure on 2+ rows
	if rows.Next() {
		return errors.New("multiple rows returned")
	}

	return nil
}
