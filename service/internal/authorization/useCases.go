package authorization

// Use Case for generating Security Contexts.
type SecurityContextGenerationUseCase interface {
	// Generate a Security Context for the given Principal.
	Generate(principal PrincipalID) SecurityContext
}

// Use Case for working with Signed Security Contexts.
type SecurityContextSigningUseCase interface {
	// Sign the provided Security Context.
	Sign(sc SecurityContext) SignedSecurityContext

	// Verify the provided Signed Security Context.
	Verify(sc SignedSecurityContext) SecurityContext
}
