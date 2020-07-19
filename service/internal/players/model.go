package players

import "github.com/sazzer/mire/service/internal/model"

// Authentication represents the authentication details of a player at an external authentication provider.
type Authentication struct {
	SystemID    string
	PlayerID    string
	DisplayName string
}

// PlayerData represents the actual data about a player.
type PlayerData struct {
	Email           string
	DisplayName     string
	Authentications []Authentication
}

// Player represents the model data for a saved player.
type Player struct {
	Identity model.Identity
	Data     PlayerData
}
