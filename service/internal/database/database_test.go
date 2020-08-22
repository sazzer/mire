package database_test

import (
	"context"
	"database/sql/driver"
	"fmt"
	"testing"

	"github.com/sazzer/mire/service/internal/database"
	"github.com/sazzer/mire/service/tests"
	"github.com/stretchr/testify/assert"
)

func TestQueryRow(t *testing.T) {
	wrapper := tests.NewDatabase(t)
	defer wrapper.Close(t)

	db := database.NewDatabase(wrapper.URL(t))

	var output struct {
		C int `db:"c"`
	}

	err := db.QueryOne(context.Background(), &output, "SELECT 1 AS c")
	assert.NoError(t, err)
	assert.Equal(t, 1, output.C)
}

func TestQueryRowMultipleFields(t *testing.T) {
	wrapper := tests.NewDatabase(t)
	defer wrapper.Close(t)

	db := database.NewDatabase(wrapper.URL(t))

	var output struct {
		C    int    `db:"c"`
		Name string `db:"name"`
	}

	err := db.QueryOne(context.Background(), &output, "SELECT 1 AS c, 'Graham' AS name")
	assert.NoError(t, err)
	assert.Equal(t, 1, output.C)
	assert.Equal(t, "Graham", output.Name)
}

type InputType struct {
	value string
}

func (c *InputType) Value() (driver.Value, error) {
	return c.value, nil
}

func TestQueryRowCustomTypeBind(t *testing.T) {
	wrapper := tests.NewDatabase(t)
	defer wrapper.Close(t)

	db := database.NewDatabase(wrapper.URL(t))

	var output struct {
		C int `db:"c"`
	}

	err := db.QueryOne(context.Background(), &output, "SELECT 1 AS c WHERE $1 = 'Graham'", &InputType{value: "Graham"})
	assert.NoError(t, err)
	assert.Equal(t, 1, output.C)
}

type OutputType struct {
	value string
}

func (c *OutputType) Scan(val interface{}) error {
	switch v := val.(type) {
	case string:
		c.value = v

		return nil
	default:
		return fmt.Errorf("Unsupported type: %T", v)
	}
}

func TestQueryRowCustomTypeReturn(t *testing.T) {
	wrapper := tests.NewDatabase(t)
	defer wrapper.Close(t)

	db := database.NewDatabase(wrapper.URL(t))

	var output struct {
		C OutputType `db:"c"`
	}

	err := db.QueryOne(context.Background(), &output, "SELECT 'Graham' AS c")
	assert.NoError(t, err)
	assert.Equal(t, "Graham", output.C.value)
}

func TestQueryRowBinds(t *testing.T) {
	wrapper := tests.NewDatabase(t)
	defer wrapper.Close(t)

	db := database.NewDatabase(wrapper.URL(t))

	var output struct {
		C int `db:"c"`
	}

	err := db.QueryOne(context.Background(), &output, "SELECT $1::int AS c", 5)
	assert.NoError(t, err)
	assert.Equal(t, 5, output.C)
}

func TestQueryRowNoRows(t *testing.T) {
	wrapper := tests.NewDatabase(t)
	defer wrapper.Close(t)

	db := database.NewDatabase(wrapper.URL(t))

	var output struct {
		C int `db:"c"`
	}

	err := db.QueryOne(context.Background(), &output, "SELECT 1 AS c WHERE 1 = 2")
	assert.Equal(t, database.ErrNoRows, err)
}

func TestQueryRowMultipleRows(t *testing.T) {
	wrapper := tests.NewDatabase(t)
	defer wrapper.Close(t)

	db := database.NewDatabase(wrapper.URL(t))

	var output struct {
		C int `db:"c"`
	}

	err := db.QueryOne(context.Background(), &output, "SELECT 1 AS c UNION SELECT 2 AS c")
	assert.NoError(t, err)
	assert.Equal(t, 1, output.C)
}

func TestQueryRowInvalidReturn(t *testing.T) {
	wrapper := tests.NewDatabase(t)
	defer wrapper.Close(t)

	db := database.NewDatabase(wrapper.URL(t))

	var output struct {
		C int `db:"d"`
	}

	err := db.QueryOne(context.Background(), &output, "SELECT 1 AS c")
	assert.Equal(t, database.ErrUnexpected, err)
}

func TestQueryRowMalformedQuery(t *testing.T) {
	wrapper := tests.NewDatabase(t)
	defer wrapper.Close(t)

	db := database.NewDatabase(wrapper.URL(t))

	var output struct {
		C int `db:"c"`
	}

	err := db.QueryOne(context.Background(), &output, "SELEC 1 AS c")
	assert.Equal(t, database.ErrUnexpected, err)
}
