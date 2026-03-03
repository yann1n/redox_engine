use rayon::prelude::*;
use std::any::TypeId;
use crate::component::Component;
use crate::query::iter::Query;
use crate::world::World;

/// Extension trait for parallel iteration on queries.
pub trait ParallelQuery<'a, T: Component> {
    /// Returns a parallel iterator over all components of type `T`.
    fn par_iter(&self, world: &'a World) -> impl ParallelIterator<Item = &T> + 'a;
}

impl<'a, T: Component> ParallelQuery<'a, T> for Query<'a, T> {
    fn par_iter(&self, world: &'a World) -> impl ParallelIterator<Item = &T> + 'a {
        let target_type = TypeId::of::<T>();
        let archetypes = &world.archetypes;

        // Iterate over archetype indices in parallel to avoid requiring `Sync` on `Vec<Archetype>`.
        (0..archetypes.len())
            .into_par_iter()
            .filter(move |&i| archetypes[i].table.has_component(target_type))
            .flat_map_iter(move |i| {
                let arch = &archetypes[i];
                // SAFETY: We have verified the component exists.
                // Components implement `Send + Sync`, so accessing them is safe.
                unsafe {
                    let column = arch.table.columns.get(&target_type).unwrap();
                    (0..column.len()).map(move |j| {
                        let ptr = column.get(j) as *const T;
                        &*ptr
                    })
                }
            })
    }
}