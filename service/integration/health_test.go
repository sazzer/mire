package integration_test

import (
	"net/http"
	"testing"

	"github.com/bradleyjkemp/cupaloy"
	"github.com/sazzer/mire/service/integration"
	"github.com/stretchr/testify/assert"
)

func TestHealth(t *testing.T) {
	service := integration.NewTestSubject(t)
	defer service.Close()

	res := service.Get("/health")
	assert.Equal(t, http.StatusOK, res.Code)
	assert.Equal(t, "application/json; charset=UTF-8", res.Header().Get("Content-Type"))
	cupaloy.SnapshotT(t, res.Body)
}
