use std::any::TypeId;
use hashbrown::HashMap;

/// Transitions between archetypes when components are added or removed.
/// This allows O(1) navigation through the archetype graph.
#[derive(Default)]
pub struct Edges {
    /// Maps a `TypeId` of a component being **added** to the destination archetype index.
    pub add: HashMap<TypeId, usize>,
    /// Maps a `TypeId` of a component being **removed** to the destination archetype index.
    pub remove: HashMap<TypeId, usize>,
}

impl Edges {
    /// Creates a new empty `Edges` structure.
    pub fn new() -> Self {
        Self {
            add: HashMap::new(),
            remove: HashMap::new(),
        }
    }
}