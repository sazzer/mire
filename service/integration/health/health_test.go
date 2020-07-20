package health_test

import (
	"net/http"
	"testing"

	"github.com/bradleyjkemp/cupaloy"
	"github.com/sazzer/mire/service/integration"
)

func TestHealth(t *testing.T) {
	service := integration.NewTestSubject(t)
	defer service.Close()

	res := service.Get("/health")

	res.AssertStatusCode(http.StatusOK)
	cupaloy.SnapshotT(t, res.Body)
}
