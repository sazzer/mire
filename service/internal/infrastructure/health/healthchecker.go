package health

// Healthchecker represents any component that can report on its own health
type Healthchecker interface {
	// CheckHealth will check the health of the component and return an error if the component is unhealthy
	CheckHealth() error
}
