use super::Identity;

/// Representation of some model resource that has been persisted
#[derive(Debug)]
pub struct Model<I, D> {
    /// The identity of the resource
    pub identity: Identity<I>,
    /// The data of the resource
    pub data: D,
}
