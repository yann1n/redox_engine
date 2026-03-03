use std::any::TypeId;

/// Trait implemented by all component types.
///
/// Components must be thread-safe (`Send + Sync`) and have a static lifetime.
pub trait Component: 'static + Send + Sync {}

// Blanket implementation for all types that satisfy the requirements.
impl<T: 'static + Send + Sync> Component for T {}

/// Metadata about a component type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComponentInfo {
    pub type_id: TypeId,
    pub size: usize,
    pub align: usize,
}

impl ComponentInfo {
    /// Creates `ComponentInfo` for the given type `T`.
    pub fn of<T: Component>() -> Self {
        Self {
            type_id: TypeId::of::<T>(),
            size: std::mem::size_of::<T>(),
            align: std::mem::align_of::<T>(),
        }
    }
}