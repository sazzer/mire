use std::ops::Deref;

/// Representation of a page of results
#[derive(Debug)]
pub struct Page<D> {
    /// The actual data in this page
    pub data: Vec<D>,
    /// The offset of the first record in the overall resultset
    pub offset: usize,
    /// The total number of records in the resultset
    pub total: usize,
}

impl<D> From<Vec<D>> for Page<D> {
    fn from(data: Vec<D>) -> Self {
        let total = data.len();
        Self {
            data,
            offset: 0,
            total,
        }
    }
}

impl<D> Deref for Page<D> {
    type Target = Vec<D>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
