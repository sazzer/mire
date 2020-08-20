package users

import "errors"

// ErrUnknownUser indicates that the requested user could not be found.
var ErrUnknownUser = errors.New("unknown user")

// GetUserUseCase represents the ability to fetch a user from the users service by ID.
type GetUserUseCase interface {
	// Get the single User from the service that has the given ID.
	GetUser(id UserID) (UserModel, error)
}

func (s service) GetUser(id UserID) (UserModel, error) {
	return UserModel{}, ErrUnknownUser
}
