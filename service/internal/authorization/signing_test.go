package authorization_test

import (
	"testing"
	"time"

	"github.com/sazzer/mire/service/internal/authorization"

	"github.com/benbjohnson/clock"
)

func TestSignSecurityContext(t *testing.T) {
	clock := clock.NewMock()
	service := authorization.NewComponent(clock, 24*time.Hour, "Key").Service
	principal := authorization.PrincipalID("testId")

	securityContext := service.Generate(principal)
	service.Sign(securityContext)
}
