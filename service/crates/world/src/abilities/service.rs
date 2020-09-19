use super::repository::AbilitiesRepository;

/// Service layer for interacting with Ability data
pub struct AbilitiesService {
    /// The repository for accessing ability data
    #[allow(dead_code)]
    pub(crate) repository: AbilitiesRepository,
}
