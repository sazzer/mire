package database

import (
	"errors"

	"github.com/jackc/pgerrcode"
	"github.com/jackc/pgx"
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

type ConstraintViolationError struct {
	cause      error
	Constraint string
}

func (e ConstraintViolationError) Error() string {
	return "mire: db: constraint violation error"
}

func (e ConstraintViolationError) Unwrap() error {
	return e.cause
}

func translateError(err error) error {
	if e, ok := err.(pgx.PgError); ok {
		log.Warn().Err(err).Str("code", e.Code).Msg("Postgres error")

		if e.Code == pgerrcode.IntegrityConstraintViolation ||
			e.Code == pgerrcode.RestrictViolation ||
			e.Code == pgerrcode.NotNullViolation ||
			e.Code == pgerrcode.ForeignKeyViolation ||
			e.Code == pgerrcode.UniqueViolation ||
			e.Code == pgerrcode.CheckViolation ||
			e.Code == pgerrcode.ExclusionViolation {
			log.Warn().Err(err).Msg("Constraint violation error")
			return ConstraintViolationError{err, e.ConstraintName}
		}
	}

	return UnexpectedError{err}
}
