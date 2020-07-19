package service

// PlayerService is the service layer for interacting with players.
type PlayerService struct {
	repository PlayerRepository
}

// New creates a new player service backed by the provided repository.
func New(repository PlayerRepository) PlayerService {
	return PlayerService{repository}
}
