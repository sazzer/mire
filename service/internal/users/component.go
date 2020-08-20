package users

import "github.com/sazzer/mire/service/internal/database"

// Component represents the users component.
type Component struct {
	Service service
}

// NewComponent creates a new instance of the authorization component.
func NewComponent(database database.Database) Component {
	repository := newRepository(database)

	return Component{
		Service: newService(repository),
	}
}
