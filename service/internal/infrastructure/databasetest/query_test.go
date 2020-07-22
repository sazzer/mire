package databasetest_test

import (
	"context"
	"errors"
	"testing"

	"github.com/sazzer/mire/service/internal/infrastructure/database"
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

func TestQueryOneNoParamsCountSuccess(t *testing.T) {
	suite := databasesuite.NewSuite(t)
	defer suite.Close(t)

	var output struct {
		C int `db:"c"`
	}

	err := suite.Database.QueryOneOrdered(context.Background(), &output, "SELECT 1 AS c")

	assert.NoError(t, err)
	assert.Equal(t, 1, output.C)
}

func TestQueryOneErrorNoRows(t *testing.T) {
	suite := databasesuite.NewSuite(t)
	defer suite.Close(t)

	var output struct {
		C int `db:"c"`
	}

	err := suite.Database.QueryOneOrdered(context.Background(), &output, "SELECT 1 AS c WHERE 1 = 2")

	assert.Error(t, err)
	assert.Equal(t, database.ErrNoRows, err)
}

func TestQueryOneErrorMultipleRows(t *testing.T) {
	suite := databasesuite.NewSuite(t)
	defer suite.Close(t)

	var output struct {
		C int `db:"c"`
	}

	err := suite.Database.QueryOneOrdered(context.Background(), &output, "SELECT 1 AS c UNION SELECT 2 AS c")

	assert.Error(t, err)
	assert.Equal(t, database.ErrMultipleRows, err)
}

func TestQueryOneErrorMalformedQuery(t *testing.T) {
	suite := databasesuite.NewSuite(t)
	defer suite.Close(t)

	var output struct {
		C int `db:"c"`
	}

	err := suite.Database.QueryOneOrdered(context.Background(), &output, "SELEC 1 AS c")

	assert.Error(t, err)
	assert.True(t, errors.As(err, &database.UnexpectedError{}))
}

func TestExecNoParamsSuccess(t *testing.T) {
	suite := databasesuite.NewSuite(t)
	defer suite.Close(t)

	count, err := suite.Database.ExecOneOrdered(context.Background(), "CREATE TABLE testing(id SERIAL PRIMARY KEY)")

	assert.NoError(t, err)
	assert.Equal(t, int64(0), count)

	var output struct {
		C int `db:"c"`
	}

	err = suite.Database.QueryOneOrdered(context.Background(), &output, "SELECT COUNT(*) AS c FROM testing")

	assert.NoError(t, err)
	assert.Equal(t, 0, output.C)
}

func TestExecNoParamsMalformed(t *testing.T) {
	suite := databasesuite.NewSuite(t)
	defer suite.Close(t)

	_, err := suite.Database.ExecOneOrdered(context.Background(), "CREATE TABLE testing")

	assert.True(t, errors.As(err, &database.UnexpectedError{}))
}

func TestExecNoParamsConstraintViolation(t *testing.T) {
	suite := databasesuite.NewSuite(t)
	defer suite.Close(t)

	_, err := suite.Database.ExecOneOrdered(context.Background(), "CREATE TABLE testing(id INT PRIMARY KEY)")
	assert.NoError(t, err)

	count, err := suite.Database.ExecOneOrdered(context.Background(), "INSERT INTO testing(id) VALUES (1)")
	assert.NoError(t, err)
	assert.Equal(t, int64(1), count)

	_, err = suite.Database.ExecOneOrdered(context.Background(), "INSERT INTO testing(id) VALUES (1)")
	assert.True(t, errors.As(err, &database.ConstraintViolationError{}))

	e, _ := err.(database.ConstraintViolationError)
	assert.Equal(t, "testing_pkey", e.Constraint)
}
