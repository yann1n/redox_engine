use smallvec::SmallVec;
use crate::entity::Entity;

/// Component added to an entity to point to its parent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Parent(pub Entity);

/// Component added to an entity to track its children.
#[derive(Debug, Clone)]
pub struct Children {
    pub list: SmallVec<[Entity; 8]>,
}

impl Children {
    /// Creates an empty children list.
    pub fn new() -> Self {
        Self {
            list: SmallVec::new(),
        }
    }
}