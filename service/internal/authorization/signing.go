package authorization

import (
	"bytes"

	"github.com/lestrrat-go/jwx/jwa"
	"github.com/lestrrat-go/jwx/jwt"
	"github.com/rs/zerolog/log"
)

// Use Case for working with Signed Security Contexts.
type SecurityContextSigningUseCase interface {
	// Sign the provided Security Context.
	Sign(sc SecurityContext) SignedSecurityContext

	// Verify the provided Signed Security Context.
	Verify(sc SignedSecurityContext) (SecurityContext, error)
}

// Sign the provided Security Context.
func (s service) Sign(sc SecurityContext) SignedSecurityContext {
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
func (s service) Verify(sc SignedSecurityContext) (SecurityContext, error) {
	token, err := jwt.Parse(bytes.NewReader([]byte(sc.string())), jwt.WithVerify(jwa.HS512, s.signingKey))
	if err != nil {
		log.Debug().Err(err).Interface("signed", sc).Msg("Failed to verify security context")

		return SecurityContext{}, err
	}

	id := token.JwtID()
	principal := token.Subject()
	expires := token.Expiration()
	issued := token.IssuedAt()

	result := SecurityContext{
		ID:        SecurityContextID(id),
		Principal: PrincipalID(principal),
		Expires:   expires.UTC(),
		Issued:    issued.UTC(),
	}

	log.Debug().Interface("securityContext", result).Interface("signed", sc).Msg("Verified security context")

	return result, nil
}
