package health

// Status represents the status of the system of a single component.
type Status string

const (
	StatusHealthy   Status = "Healthy"
	StatusUnhealthy Status = "Unhealthy"
)

//////////
// The health of the individual components
//////////

// ComponentHealth represents the health of a single component.
type ComponentHealth struct {
	Error error
}

// Status returns the health status of this component.
func (c ComponentHealth) Status() Status {
	if c.Error == nil {
		return StatusHealthy
	}

	return StatusUnhealthy
}

//////////
// The health of the whole system
//////////

// SystemHealth represents the health of the entire system.
type SystemHealth struct {
	Components map[string]ComponentHealth
}

// Status returns the health status of the whole system.
func (c SystemHealth) Status() Status {
	result := StatusHealthy

	for _, component := range c.Components {
		if component.Status() == StatusUnhealthy {
			result = StatusUnhealthy
		}
	}

	return result
}
