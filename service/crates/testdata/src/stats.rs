use super::SeedData;
use postgres_types::ToSql;
use uuid::Uuid;

#[derive(Debug)]
pub struct SeedStat {
    pub stat_id: Uuid,
    pub name: String,
    pub description: String,
    pub default_value: i32,
}

impl Default for SeedStat {
    fn default() -> Self {
        Self {
            stat_id: Uuid::new_v4(),
            name: format!("Stat {}", Uuid::new_v4()),
            description: format!("Description {}", Uuid::new_v4()),
            default_value: 0,
        }
    }
}

impl SeedData for SeedStat {
    fn sql(&self) -> &str {
        "INSERT INTO stats(stat_id, name, description, default_value) VALUES ($1, $2, $3, $4)"
    }

    fn binds(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.stat_id,
            &self.name,
            &self.description,
            &self.default_value,
        ]
    }
}
