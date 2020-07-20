package databasetest_test

import (
	"context"
	"testing"

	"github.com/sazzer/mire/service/test/databasesuite"
	"github.com/stretchr/testify/assert"
)

func TestHealthcheck(t *testing.T) {
	suite := databasesuite.NewSuite(t)
	defer suite.Close(t)

	err := suite.Database.Healthcheck(context.Background())
	assert.NoError(t, err)
}
