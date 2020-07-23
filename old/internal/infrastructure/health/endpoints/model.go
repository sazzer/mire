package endpoints

// StatusModel is a type representing the status of a component or the system.
type StatusModel string

const (
	// StatusHealthy means the component is healthy
	StatusHealthy StatusModel = "HEALTHY"
	// StatusUnhealthy means the component is unhealthy
	StatusUnhealthy StatusModel = "UNHEALTHY"
)

// ComponentHealthModel is the model representation of a single component.
type ComponentHealthModel struct {
	Status  StatusModel `json:"status"`
	Message string      `json:"message,omitempty"`
}

// SystemHealthModel is the model representation of the entire system.
type SystemHealthModel struct {
	Status     StatusModel                     `json:"status"`
	Components map[string]ComponentHealthModel `json:"components"`
}
