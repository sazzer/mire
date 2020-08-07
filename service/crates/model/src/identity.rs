use chrono::{DateTime, Utc};
use uuid::Uuid;

/// The identity of some model resource
#[derive(Debug, PartialEq)]
pub struct Identity<I> {
    /// The actual ID of the resource
    pub id: I,
    /// A version tag of the resource
    pub version: Uuid,
    /// When the resource was created
    pub created: DateTime<Utc>,
    /// When the resource was last updated
    pub updated: DateTime<Utc>,
}

impl<I> Identity<I> {
    /// Create a new Identity instance for the given ID value to represent a brand new resource
    ///
    /// # Parameters
    /// - `id` - The ID of the new resource
    ///
    /// # Returns
    /// The Identity object
    pub fn new(id: I) -> Self {
        let now = Utc::now();

        Self {
            id,
            version: Uuid::new_v4(),
            created: now,
            updated: now,
        }
    }
}

impl<I> Default for Identity<I>
where
    I: Default,
{
    /// Create a new Identity instance with a default ID value to represent a brand new resource
    ///
    /// # Returns
    /// The Identity object
    fn default() -> Self {
        Self::new(I::default())
    }
}
