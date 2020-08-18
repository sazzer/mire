package authorization_test

import (
	"testing"
	"time"

	"github.com/stretchr/testify/assert"

	"github.com/sazzer/mire/service/internal/authorization"

	"github.com/benbjohnson/clock"
)

func TestGenerateSecurityContext(t *testing.T) {
	clock := clock.NewMock()
	service := authorization.NewComponent(clock, 24*time.Hour, "Key").Service
	principal := authorization.PrincipalID("testId")

	securityContext := service.Generate(principal)

	assert.Equal(t, principal, securityContext.Principal)
	assert.Equal(t, clock.Now().UTC(), securityContext.Issued)
	assert.Equal(t, clock.Now().UTC().Add(24*time.Hour), securityContext.Expires)
}
