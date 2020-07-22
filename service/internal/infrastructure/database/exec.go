package database

import (
	"context"
	"database/sql"

	"github.com/rs/zerolog/log"
)

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

func (d Database) ExecOneOrdered(ctx context.Context, query string, binds ...interface{}) (int64, error) {
	log.Debug().Str("sql", query).Interface("binds", binds).Msg("Executing query")

	return d.exec(ctx, func(ctx context.Context) (sql.Result, error) {
		return d.db.ExecContext(ctx, query, binds...)
	})
}
