package health

import "github.com/go-chi/chi"

// Component represents the actual Health checking component.
type Component struct {
}

// NewComponent creates a new instance of the health checking component.
func NewComponent() Component {
	return Component{}
}

// RegisterRoutes will ensure that the healthchecker routes are registered with the server.
func (c Component) RegisterRoutes(r chi.Router) {
	http := HTTP{}
	r.Get("/health", http.getHealth)
}
