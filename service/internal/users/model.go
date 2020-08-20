package users

import "time"

// The ID of a user in the system.
type UserID struct {
	value string
}

// Construct a new User ID object wrapping the provided value.
func NewUserID(value string) (UserID, error) {
	return UserID{value}, nil
}

// The Email Address of a user in the system.
type Email struct {
	value string
}

// Construct a new Email object wrapping the provided value.
func NewEmail(value string) (Email, error) {
	return Email{value}, nil
}

// The ID of the user at a specific authentication provider.
type AuthenticationID struct {
	value string
}

// Construct a new AuthenticationID object wrapping the provided value.
func NewAuthenticationID(value string) (AuthenticationID, error) {
	return AuthenticationID{value}, nil
}

// The ID of the authentication provider the user is registered at.
type AuthenticationProviderID struct {
	value string
}

// Construct a new AuthenticationProviderID object wrapping the provided value.
func NewAuthenticationProviderID(value string) (AuthenticationProviderID, error) {
	return AuthenticationProviderID{value}, nil
}

// The details of a user at a single authentication provider.
type Authentication struct {
	ID          AuthenticationID
	Provider    AuthenticationProviderID
	DisplayName string
}

// The data that represents a user record.
type UserData struct {
	Email           Email
	DisplayName     string
	Authentications []Authentication
}

// The data that represents a persisted user in the service.
type UserModel struct {
	ID      UserID
	Version string
	Created time.Time
	Updated time.Time
	UserData
}
