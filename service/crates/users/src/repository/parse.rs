use crate::{UserData, UserModel};
use mire_model::Identity;
use tokio_postgres::row::Row;

#[allow(clippy::fallible_impl_from)]
impl From<Row> for UserModel {
    fn from(row: Row) -> Self {
        Self {
            identity: Identity {
                id: row.get("user_id"),
                version: row.get("version"),
                created: row.get("created"),
                updated: row.get("updated"),
            },
            data: UserData {
                email: row.get("email"),
                display_name: row.get("display_name"),
                authentications: serde_json::from_value(row.get("authentications")).unwrap(),
            },
        }
    }
}
