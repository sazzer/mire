package integration

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/sazzer/mire/service/internal"
	"github.com/sazzer/mire/service/test"
	"github.com/stretchr/testify/assert"
)

// TestSubject represents a wrapper around the entire service we are testing.
type TestSubject struct {
	t        *testing.T
	database test.Database
	service  internal.Mire
}

// NewTestSubject creates a new test subject for us to test.
func NewTestSubject(t *testing.T) TestSubject {
	database := test.NewDatabase(t)

	return TestSubject{
		t:        t,
		database: database,
		service:  internal.New(database.URL(t)),
	}
}

func (t *TestSubject) Close() {
	t.database.Close(t.t)
}

// Inject will send the given HTTP Request to the service and return the response.
func (t *TestSubject) Inject(req *http.Request) *httptest.ResponseRecorder {
	return t.service.Inject(req)
}

// Get is a helper to perform a GET request to the given URL.
func (t *TestSubject) Get(url string) *httptest.ResponseRecorder {
	ctx, cancel := context.WithTimeout(context.Background(), 500*time.Millisecond)
	defer cancel()

	req, err := http.NewRequestWithContext(ctx, "GET", url, nil)
	assert.NoError(t.t, err)

	return t.Inject(req)
}
