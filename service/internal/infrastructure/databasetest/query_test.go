package databasetest_test

import (
	"context"
	"testing"

	"github.com/sazzer/mire/service/test/databasesuite"
	"github.com/stretchr/testify/assert"
)

func TestQueryOneNamedCountSuccess(t *testing.T) {
	suite := databasesuite.NewSuite(t)
	defer suite.Close(t)

	var output struct {
		C int `db:"c"`
	}

	err := suite.Database.QueryOneNamed(context.Background(), &output, "SELECT 1 AS c WHERE :a = 1",
		map[string]interface{}{
			"a": 1,
		})

	assert.NoError(t, err)
	assert.Equal(t, 1, output.C)
}

func TestQueryOneOrderedCountSuccess(t *testing.T) {
	suite := databasesuite.NewSuite(t)
	defer suite.Close(t)

	var output struct {
		C int `db:"c"`
	}

	err := suite.Database.QueryOneOrdered(context.Background(), &output, "SELECT 1 AS c WHERE $1 = 1", 1)

	assert.NoError(t, err)
	assert.Equal(t, 1, output.C)
}
