package database

import (
	"context"
	"database/sql"

	"github.com/rs/zerolog/log"
)

// exec will use the provided lambda function to execute a query against the database and then handle the
// result of this. This will either return the number of rows updated, or an error if something went wrong.
func (d Database) exec(ctx context.Context, f func(context.Context) (sql.Result, error)) (int64, error) {
	result, err := f(ctx)
	if err != nil {
		log.Warn().Err(err).Msg("Failed to execute statement")
		return 0, translateError(err)
	}

	numberOfRows, err := result.RowsAffected()
	if err != nil {
		log.Warn().Err(err).Msg("Failed to get number of rows affected")
		return 0, translateError(err)
	}

	log.Debug().Int64("numberOfRows", numberOfRows).Msg("Number of rows affected")

	return numberOfRows, nil
}

// ExecOrdered will execute a statement against the database where the binds are an ordered varargs list.
// This expects the query to have ordered placeholders - $1, $2, etc - as appropriate.
func (d Database) ExecOrdered(ctx context.Context, query string, binds ...interface{}) (int64, error) {
	log.Debug().Str("sql", query).Interface("binds", binds).Msg("Executing query")

	return d.exec(ctx, func(ctx context.Context) (sql.Result, error) {
		return d.db.ExecContext(ctx, query, binds...)
	})
}
