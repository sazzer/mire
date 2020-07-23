package players

import "context"

// service represents the actual service layer for working with players.
type Service struct {
	repository repository
}

// Create will create a new player with the given data.
func (s *Service) Create(ctx context.Context, data PlayerData) (Player, error) {
	return s.repository.Create(ctx, data)
}
