package authorization

import "github.com/google/uuid"

// Use Case for generating Security Contexts.
type SecurityContextGenerationUseCase interface {
	// Generate a Security Context for the given Principal.
	Generate(principal PrincipalID) SecurityContext
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
