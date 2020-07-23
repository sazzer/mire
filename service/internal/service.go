package internal

// Mire represents the actual service
type Mire struct{}

// New creates a new instance of the Mire service
func New(databaseURL string) Mire {
	return Mire{}
}

// Start will start the Mire service listening on the given port
func (m *Mire) Start(port uint16) {

}
