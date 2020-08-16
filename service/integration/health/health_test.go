package health_test

import (
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/bradleyjkemp/cupaloy"
	"github.com/sazzer/mire/service/integration"
	"gotest.tools/assert"
)

func TestHealth(t *testing.T) {
	service := integration.NewService(t)
	defer service.Close()

	req := httptest.NewRequest("GET", "/health", nil)

	res := service.Inject(req)

	assert.Equal(t, http.StatusOK, res.StatusCode)
	cupaloy.SnapshotT(t, res.Body)
}
