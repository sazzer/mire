package authorization_test

import (
	"testing"
	"time"

	"github.com/sazzer/mire/service/internal/authorization"

	"github.com/benbjohnson/clock"
)

func TestSignSecurityContext(t *testing.T) {
	clock := clock.NewMock()
	service := authorization.NewService(clock, 24*time.Hour, "Key")
	principal := authorization.PrincipalID("testId")

	securityContext := service.Generate(principal)
	service.Sign(securityContext)
}
