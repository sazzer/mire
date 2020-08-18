package authorization

import (
	"time"

	"github.com/rs/zerolog/log"

	"github.com/google/uuid"
	"github.com/lestrrat-go/jwx/jwa"
	"github.com/lestrrat-go/jwx/jwt"

	"github.com/benbjohnson/clock"
)

// Service layer for working with security contexts.
type Service struct {
	clock      clock.Clock
	duration   time.Duration
	signingKey []byte
}

// Create a new instance of the service layer.
func NewService(clock clock.Clock, duration time.Duration, signingKey string) Service {
	return Service{
		clock:      clock,
		duration:   duration,
		signingKey: []byte(signingKey),
	}
}

// Generate a Security Context for the given Principal.
func (s *Service) Generate(principal PrincipalID) SecurityContext {
	id := SecurityContextID(uuid.New().String())
	issued := s.clock.Now().UTC()
	expires := issued.Add(s.duration)

	return SecurityContext{
		ID:        id,
		Issued:    issued,
		Expires:   expires,
		Principal: principal,
	}
}

// Sign the provided Security Context.
func (s *Service) Sign(sc SecurityContext) SignedSecurityContext {
	token := jwt.New()
	_ = token.Set(jwt.IssuerKey, "tag:mire,2020:authorization")
	_ = token.Set(jwt.AudienceKey, "tag:mire,2020:authorization")
	_ = token.Set(jwt.JwtIDKey, sc.ID.string())
	_ = token.Set(jwt.SubjectKey, sc.Principal.string())
	_ = token.Set(jwt.ExpirationKey, sc.Expires)
	_ = token.Set(jwt.IssuedAtKey, sc.Issued)
	_ = token.Set(jwt.NotBeforeKey, sc.Issued)

	signed, err := jwt.Sign(token, jwa.HS512, s.signingKey)
	if err != nil {
		log.Fatal().Err(err).Msg("Failed to sign security context")
	}

	result := SignedSecurityContext(string(signed))
	log.Debug().Interface("securityContext", sc).Interface("signed", result).Msg("Signed security context")

	return result
}

// Verify the provided Signed Security Context.
func (s *Service) Verify(sc SignedSecurityContext) (SecurityContext, error) {
	return SecurityContext{}, nil
}
