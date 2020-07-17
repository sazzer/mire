package health_test

import (
	"net/http"
	"testing"

	"github.com/bradleyjkemp/cupaloy"
	"github.com/sazzer/mire/service/tests"
)

func TestHealth(t *testing.T) {
	service := tests.NewTestSubject(t)

	res := service.Get("/health")

	res.AssertStatusCode(http.StatusOK)
	cupaloy.SnapshotT(t, res.Body)
}
