package authorization_test

import (
	"testing"
	"time"

	"github.com/sazzer/mire/service/internal/authorization"
	"github.com/stretchr/testify/assert"

	"github.com/benbjohnson/clock"
)

func TestSignSecurityContext(t *testing.T) {
	clock := clock.NewMock()
	service := authorization.NewComponent(clock, 24*time.Hour, "Key").Service
	principal := authorization.PrincipalID("testId")

	securityContext := service.Generate(principal)
	service.Sign(securityContext)
}

func TestVerifySecurityContext(t *testing.T) {
	clock := clock.NewMock()
	service := authorization.NewComponent(clock, 24*time.Hour, "Key").Service
	principal := authorization.PrincipalID("testId")

	securityContext := service.Generate(principal)
	signed := service.Sign(securityContext)

	verified, err := service.Verify(signed)
	assert.NoError(t, err)
	assert.Equal(t, securityContext, verified)
}

func TestVerifyMalformedSecurityContext(t *testing.T) {
	clock := clock.NewMock()
	service := authorization.NewComponent(clock, 24*time.Hour, "Key").Service

	signed := authorization.SignedSecurityContext("malformed")

	_, err := service.Verify(signed)
	assert.Error(t, err)
}
