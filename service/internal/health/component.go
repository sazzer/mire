package health

import "github.com/go-chi/chi"

// Component represents the actual Health checking component.
type Component struct {
	service Service
}

// NewComponent creates a new instance of the health checking component.
func NewComponent() Component {
	return Component{
		service: Service{},
	}
}

func (c Component) WithComponent(name string, component Healthchecker) Component {
	c.service.WithComponent(name, component)

	return c
}

// RegisterRoutes will ensure that the healthchecker routes are registered with the server.
func (c Component) RegisterRoutes(r chi.Router) {
	http := HTTP(c)

	r.Get("/health", http.getHealth)
}
