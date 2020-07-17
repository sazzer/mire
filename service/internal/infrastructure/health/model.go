package health

// Status represents the status of the system of a single component
type Status string

const (
	// StatusHealthy indicates that the status is healthy
	StatusHealthy Status = "Healthy"
	// StatusUnhealthy indicates that the status is unhealthy
	StatusUnhealthy Status = "Unhealthy"
)

//////////
// The health of the individual components
//////////

// ComponentHealth represents the health of a single component
type ComponentHealth struct {
	error error
}

// NewComponentHealth creates a new Component Health
func NewComponentHealth(error error) ComponentHealth {
	return ComponentHealth{error: error}
}

// Status returns the health status of this component
func (c ComponentHealth) Status() Status {
	if c.error == nil {
		return StatusHealthy
	}
	return StatusUnhealthy
}

// Error returns the error details for this component, if there are any
func (c ComponentHealth) Error() error {
	return c.error
}

//////////
// The health of the whole system
//////////

// SystemHealth represents the health of the entire system
type SystemHealth struct {
	components map[string]ComponentHealth
}

// NewSystemHealth creates a new System Health response for the provided components
func NewSystemHealth(components map[string]ComponentHealth) SystemHealth {
	return SystemHealth{components: components}
}

// Status returns the health status of the whole system
func (c SystemHealth) Status() Status {
	result := StatusHealthy
	for _, component := range c.components {
		if component.Status() == StatusUnhealthy {
			result = StatusUnhealthy
		}
	}
	return result
}

// Components returns the health status of the individual components in the system
func (c SystemHealth) Components() map[string]ComponentHealth {
	return c.components
}
