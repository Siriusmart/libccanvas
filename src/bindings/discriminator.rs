use serde::{Deserialize, Serialize};

/// unique identifier every component has
#[derive(Default, PartialEq, Eq, Clone, Debug, Deserialize, Serialize, Hash)]
pub struct Discriminator(Vec<u32>);

impl Discriminator {
    /// construct new self
    /// so the discrim cannot be modified
    pub fn new(path: Vec<u32>) -> Self {
        Self(path)
    }
}
