package database

import (
	"context"
	"database/sql"
	"sort"

	"github.com/gobuffalo/packr/v2"
	"github.com/jmoiron/sqlx"
	"github.com/rs/zerolog/log"
)

// Migrate the database to the latest version of the schema.
func Migrate(d Database) {
	ctx := context.Background()

	log.Info().Msg("Migrating database schema")

	tx, err := d.db.BeginTxx(ctx, &sql.TxOptions{
		Isolation: sql.LevelSerializable,
		ReadOnly:  false,
	})
	if err != nil {
		log.Fatal().Err(err).Msg("Failed to start transaction")
	}

	createMigrationsTable(ctx, tx)
	appliedMigrations := loadAppliedMigrations(ctx, tx)

	box := packr.New("Migrations", "./migrations")
	files := box.List()
	log.Info().Strs("migrations", files).Msg("Available migrations")

	for _, file := range files {
		if !contains(appliedMigrations, file) {
			applyMigration(ctx, tx, box, file)
		}
	}

	err = tx.Commit()
	if err != nil {
		log.Fatal().Err(err).Msg("Failed to commit transaction")
	}
}

// createMigrationsTable will ensure that the __migrations table exists.
func createMigrationsTable(ctx context.Context, tx *sqlx.Tx) {
	_, err := tx.ExecContext(ctx, `CREATE TABLE IF NOT EXISTS __migrations(
		migration_file TEXT PRIMARY KEY,
		sequence SERIAL NOT NULL,
		executed TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
		executed_from TEXT NOT NULL DEFAULT inet_client_addr()
	)`)
	if err != nil {
		log.Fatal().Err(err).Msg("Failed to create migrations table")
	}
}

// loadAppliedMigrations will load from the database the list of migrations that have been previously applied.
func loadAppliedMigrations(ctx context.Context, tx *sqlx.Tx) []string {
	migrations := []string{}

	err := tx.SelectContext(ctx, &migrations, "SELECT migration_file FROM __migrations")
	if err != nil {
		log.Fatal().Err(err).Msg("Failed to load applied migrations")
	}

	sort.Strings(migrations)

	log.Info().Strs("migrations", migrations).Msg("Applied migrations")

	return migrations
}

// applyMigration will actually apply a migration and record that it was applied.
func applyMigration(ctx context.Context, tx *sqlx.Tx, box *packr.Box, file string) {
	log.Info().Str("filename", file).Msg("Processing migration file")

	contents, err := box.FindString(file)
	if err != nil {
		log.Fatal().Err(err).Str("file", file).Msg("Failed to load migration file")
	}

	_, err = tx.ExecContext(ctx, contents)
	if err != nil {
		log.Fatal().Err(err).Str("file", file).Msg("Failed to apply migration file")
	}

	_, err = tx.ExecContext(ctx, "INSERT INTO __migrations(migration_file) VALUES ($1)", file)
	if err != nil {
		log.Fatal().Err(err).Str("file", file).Msg("Failed to record migration file applied")
	}
}

func contains(s []string, searchterm string) bool {
	i := sort.SearchStrings(s, searchterm)

	return i < len(s) && s[i] == searchterm
}
