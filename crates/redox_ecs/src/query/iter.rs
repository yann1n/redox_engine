use std::any::TypeId;
use std::marker::PhantomData;
use crate::world::World;
use crate::component::Component;

/// A query that fetches components of type `T` from the world.
pub struct Query<'a, T: Component> {
    _marker: PhantomData<&'a T>,
}

impl<'a, T: Component> Query<'a, T> {
    /// Creates a new query for component type `T`.
    pub fn new() -> Self {
        Self { _marker: PhantomData }
    }

    /// Returns an iterator over all components of type `T` in the world.
    pub fn iter(&self, world: &'a World) -> impl Iterator<Item = &T> + 'a {
        let target_type = TypeId::of::<T>();

        world.archetypes.iter()
            .filter(move |arch| arch.table.has_component(target_type))
            .flat_map(move |arch| {
                unsafe {
                    let column = arch.table.columns.get(&target_type).unwrap();
                    (0..column.len()).map(move |i| {
                        let ptr = column.get(i) as *const T;
                        &*ptr
                    })
                }
            })
    }
}