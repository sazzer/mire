package health

// HealthcheckUseCase represents the actual Health Check use case
type HealthcheckUseCase interface {
	// CheckHealth will check the overall system health and return this
	CheckHealth() SystemHealth
}
