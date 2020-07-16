package server

import "github.com/labstack/echo/v4"

// Configurer is an interface that any type able to configure routes will implement
type Configurer interface {
	RegisterRoutes(e *echo.Echo) error
}
