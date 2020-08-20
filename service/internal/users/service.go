package users

// The actual implementation of the users service for working with user records.
type service struct {
	repository repository
}

// Create a new users service.
func newService(repository repository) service {
	return service{
		repository: repository,
	}
}
