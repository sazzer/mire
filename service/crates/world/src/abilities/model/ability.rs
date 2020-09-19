use super::AbilityId;

/// Representation of an Ability Score that a player has - e.g. Strength or Intelligence
pub struct Ability {
    /// The ID of the Ability Score
    pub id: AbilityId,
    /// The name of the Ability Score
    pub name: String,
    /// The description of the Ability Score
    pub description: String,
}
