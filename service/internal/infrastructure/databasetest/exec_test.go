package databasetest_test

import (
	"context"
	"errors"
	"testing"

	"github.com/sazzer/mire/service/internal/infrastructure/database"
	"github.com/sazzer/mire/service/test/databasesuite"
	"github.com/stretchr/testify/assert"
)

// TODO: Make these tests run off of a Suite

func TestExecNoParamsSuccess(t *testing.T) {
	suite := databasesuite.NewSuite(t)
	defer suite.Close(t)

	count, err := suite.Database.ExecOrdered(context.Background(), "CREATE TABLE testing(id SERIAL PRIMARY KEY)")

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

	_, err := suite.Database.ExecOrdered(context.Background(), "CREATE TABLE testing")

	assert.True(t, errors.As(err, &database.UnexpectedError{}))
}

func TestExecNoParamsConstraintViolation(t *testing.T) {
	suite := databasesuite.NewSuite(t)
	defer suite.Close(t)

	_, err := suite.Database.ExecOrdered(context.Background(), "CREATE TABLE testing(id INT PRIMARY KEY)")
	assert.NoError(t, err)

	count, err := suite.Database.ExecOrdered(context.Background(), "INSERT INTO testing(id) VALUES (1)")
	assert.NoError(t, err)
	assert.Equal(t, int64(1), count)

	_, err = suite.Database.ExecOrdered(context.Background(), "INSERT INTO testing(id) VALUES (1)")
	assert.True(t, errors.As(err, &database.ConstraintViolationError{}))

	e, _ := err.(database.ConstraintViolationError)
	assert.Equal(t, "testing_pkey", e.Constraint)
}
