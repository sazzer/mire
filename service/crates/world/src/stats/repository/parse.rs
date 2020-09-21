use crate::Stat;
use tokio_postgres::row::Row;

impl From<Row> for Stat {
    #[allow(clippy::cast_sign_loss)]
    fn from(row: Row) -> Self {
        let default_value: i32 = row.get("default_value");

        Self {
            id: row.get("stat_id"),
            name: row.get("name"),
            description: row.get("description"),
            default_value: default_value as u32,
        }
    }
}
