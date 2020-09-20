use crate::Stat;
use tokio_postgres::row::Row;

impl From<Row> for Stat {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("stat_id"),
            name: row.get("name"),
            description: row.get("description"),
            default_value: row.get("default_value"),
        }
    }
}
