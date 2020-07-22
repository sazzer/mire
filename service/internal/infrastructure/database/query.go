package database

import (
	"context"

	"github.com/jmoiron/sqlx"
	"github.com/rs/zerolog/log"
)

// queryOne will use the provided lambda function to execute a query against the database and then handle the
// result of this. This expects exactly one row to be returned.
// If a database error occurs then the appropriate response - normally an UnexpectedError - is returned.
// If zero rows are returned then ErrNoRows is returned.
// If 2+ rows are returned then ErrMultipleRows is returned
// Otherwise no error is returned and the output parameter will contain the data from the row that did come back.
func (d Database) queryOne(ctx context.Context, output interface{}, f func(context.Context) (*sqlx.Rows, error)) error {
	rows, err := f(ctx)
	if err != nil {
		log.Warn().Err(err).Msg("Failed to execute query")
		return translateError(err)
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
		return translateError(err)
	}

	log.Debug().Interface("result", output).Msg("Read value from database")

	// Failure on 2+ rows
	if rows.Next() {
		return ErrMultipleRows
	}

	return nil
}

// QueryOneNamed will execute a query against the database with named parameters and populate a single output
// parameter with the matching row. This expects exactly one row to be returned, and that the query will have
// named bind parameters - :foo, :bar, etc - matching fields in the binds parameter.
// If a database error occurs then the appropriate response - normally an UnexpectedError - is returned.
// If zero rows are returned then ErrNoRows is returned.
// If 2+ rows are returned then ErrMultipleRows is returned
// Otherwise no error is returned and the output parameter will contain the data from the row that did come back.
func (d Database) QueryOneNamed(ctx context.Context, output interface{}, query string, binds interface{}) error {
	log.Debug().Str("sql", query).Interface("binds", binds).Msg("Executing query")

	return d.queryOne(ctx, output, func(ctx context.Context) (*sqlx.Rows, error) {
		return d.db.NamedQueryContext(ctx, query, binds)
	})
}

// QueryOneNamed will execute a query against the database with ordered parameters and populate a single output
// parameter with the matching row. This expects exactly one row to be returned, and that the query will have
// ordered bind parameters - $1, $2, etc - matching fields in the varargs parameter.
// If a database error occurs then the appropriate response - normally an UnexpectedError - is returned.
// If zero rows are returned then ErrNoRows is returned.
// If 2+ rows are returned then ErrMultipleRows is returned
// Otherwise no error is returned and the output parameter will contain the data from the row that did come back.
func (d Database) QueryOneOrdered(ctx context.Context, output interface{}, query string, binds ...interface{}) error {
	log.Debug().Str("sql", query).Interface("binds", binds).Msg("Executing query")

	return d.queryOne(ctx, output, func(ctx context.Context) (*sqlx.Rows, error) {
		return d.db.QueryxContext(ctx, query, binds...)
	})
}
