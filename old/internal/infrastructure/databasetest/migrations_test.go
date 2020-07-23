package databasetest_test

import (
	"testing"

	"github.com/sazzer/mire/service/test/databasesuite"
)

func TestMigrations(t *testing.T) {
	suite := databasesuite.NewSuite(t)
	defer suite.Close(t)
}
