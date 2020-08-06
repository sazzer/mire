use super::SeedData;
use chrono::{DateTime, Timelike, Utc};
use postgres_types::ToSql;
use serde_json::json;
use uuid::Uuid;

#[derive(Debug)]
pub struct SeedUser {
    pub user_id: Uuid,
    pub version: Uuid,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub email: String,
    pub display_name: String,
    pub authentications: serde_json::Value,
}

impl Default for SeedUser {
    fn default() -> Self {
        Self {
            user_id: Uuid::new_v4(),
            version: Uuid::new_v4(),
            created: Utc::now().with_nanosecond(0).unwrap(),
            updated: Utc::now().with_nanosecond(0).unwrap(),
            email: format!("{}@example.com", Uuid::new_v4()),
            display_name: format!("{}", Uuid::new_v4()),
            authentications: json!([]),
        }
    }
}

impl SeedData for SeedUser {
    fn sql(&self) -> &str {
        "INSERT INTO users(user_id, version, created, updated, email, display_name, authentications)
        VALUES ($1, $2, $3, $4, $5, $6, $7)"
    }

    fn binds(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id,
            &self.version,
            &self.created,
            &self.updated,
            &self.email,
            &self.display_name,
            &self.authentications,
        ]
    }
}
