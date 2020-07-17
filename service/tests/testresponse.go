package tests

import (
	"net/http/httptest"
	"testing"

	"github.com/stretchr/testify/assert"
)

// TestResponse represents the response from injecting an HTTP Request.
type TestResponse struct {
	t *testing.T
	*httptest.ResponseRecorder
}

// AssertStatusCode will assert that the response represents the desired HTTP status code.
func (t *TestResponse) AssertStatusCode(statusCode int) {
	assert.Equal(t.t, statusCode, t.Code)
}
