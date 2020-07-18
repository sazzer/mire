package tests_test

import (
	"testing"

	"github.com/sazzer/mire/service/tests"
)

func TestStartService(t *testing.T) {
	service := tests.NewTestSubject(t)
	defer service.Close()
}
