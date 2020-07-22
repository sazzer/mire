package players

import "github.com/sazzer/mire/service/internal/infrastructure/database"

type repository struct {
	database database.Database
}
