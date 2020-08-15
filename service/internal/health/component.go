package health

import "github.com/go-chi/chi"

// Component represents the actual Health checking component.
type Component struct {
	service Service
}

// NewComponent creates a new instance of the health checking component.
func NewComponent(components map[string]Healthchecker) Component {
	return Component{
		service: NewService(components),
	}
}

// RegisterRoutes will ensure that the healthchecker routes are registered with the server.
func (c Component) RegisterRoutes(r chi.Router) {
	http := HTTP(c)

	r.Get("/health", http.getHealth)
}
