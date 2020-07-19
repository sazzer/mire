package server

import (
	"github.com/go-chi/chi"
)

// Configurer is an interface that any type able to configure routes will implement.
type Configurer interface {
	RegisterRoutes(r chi.Router)
}
