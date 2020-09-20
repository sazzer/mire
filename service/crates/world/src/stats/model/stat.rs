use super::StatId;

/// Representation of an Stat Score that a player has - e.g. Strength or Intelligence
pub struct Stat {
    /// The ID of the Stat Score
    pub id: StatId,
    /// The name of the Stat Score
    pub name: String,
    /// The description of the Stat Score
    pub description: String,
    /// The default value of the Stat Score
    pub default_value: u32
}
