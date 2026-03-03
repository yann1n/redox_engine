use std::any::TypeId;
use hashbrown::HashMap;
use crate::entity::{Entity, EntityAllocator};
use crate::component::{Component, ComponentInfo};
use crate::archetype::Archetype;

/// The central store of all entities and components.
///
/// Organises data into archetypes (tables) for efficient iteration.
pub struct World {
    entities: EntityAllocator,
    entity_locations: HashMap<Entity, (usize, usize)>,
    pub(crate) archetypes: Vec<Archetype>,
    type_to_archetype: HashMap<Vec<TypeId>, usize>,
}

impl World {
    /// Creates a new empty world.
    pub fn new() -> Self {
        Self {
            entities: EntityAllocator::new(),
            entity_locations: HashMap::new(),
            archetypes: Vec::new(),
            type_to_archetype: HashMap::new(),
        }
    }

    /// Spawns a new entity with no components.
    pub fn spawn(&mut self) -> Entity {
        self.entities.allocate()
    }

    /// Adds a component to an entity.
    ///
    /// If the entity already has a component of this type, it is replaced.
    /// The entity may move to a different archetype.
    pub fn add_component<T: Component>(&mut self, entity: Entity, component: T) {
        let component_info = ComponentInfo::of::<T>();

        let location = self.entity_locations.get(&entity).copied();

        let mut new_infos = if let Some((arch_idx, _)) = location {
            self.archetypes[arch_idx].component_infos.clone()
        } else {
            Vec::new()
        };

        // If component already exists, just update in place
        if new_infos.iter().any(|i| i.type_id == component_info.type_id) {
            if let Some((arch_idx, row)) = location {
                let table = &mut self.archetypes[arch_idx].table;
                unsafe {
                    let col = table.columns.get_mut(&component_info.type_id).unwrap();
                    let ptr = col.get(row) as *mut T;
                    *ptr = component;
                }
            }
            return;
        }

        // Add new component type
        new_infos.push(component_info);
        let target_arch_idx = self.find_or_create_archetype(new_infos);

        unsafe {
            if let Some((old_arch_idx, old_row)) = location {
                // SAFETY: We need two mutable references to different indices in the vector.
                // We use raw pointers to bypass the borrow checker check, verifying indices are different.

                let archetypes_ptr = self.archetypes.as_mut_ptr();
                let old_arch = &mut *archetypes_ptr.add(old_arch_idx);
                let new_arch = &mut *archetypes_ptr.add(target_arch_idx);

                new_arch.table.push_entity(entity);

                // Copy all components from old archetype to new one
                for info in &old_arch.component_infos {
                    let src_ptr = old_arch.table.get_component_ptr(info.type_id, old_row).unwrap();
                    new_arch.table.push_component_raw(info.type_id, src_ptr);
                }

                // Remove old row and update location of moved entity if any
                if let Some(moved_entity) = old_arch.table.swap_remove(old_row) {
                    self.entity_locations.insert(moved_entity, (old_arch_idx, old_row));
                }
            } else {
                self.archetypes[target_arch_idx].table.push_entity(entity);
            }

            // Add the new component
            let new_arch = &mut self.archetypes[target_arch_idx];
            let ptr = &component as *const T as *const u8;
            new_arch.table.push_component_raw(component_info.type_id, ptr);
            std::mem::forget(component);
        }

        let new_row = self.archetypes[target_arch_idx].table.row_count() - 1;
        self.entity_locations.insert(entity, (target_arch_idx, new_row));
    }

    /// Removes a component from an entity.
    ///
    /// Returns `true` if the component was present and removed, `false` otherwise.
    pub fn remove_component<T: Component>(&mut self, entity: Entity) -> bool {
        let component_info = ComponentInfo::of::<T>();
        let location = match self.entity_locations.get(&entity).copied() {
            Some(loc) => loc,
            None => return false, // entity not found
        };
        let (arch_idx, row) = location;

        // Check if the component exists
        if !self.archetypes[arch_idx].table.has_component(component_info.type_id) {
            return false;
        }

        // Build new component set without this component
        let old_infos = &self.archetypes[arch_idx].component_infos;
        let new_infos: Vec<_> = old_infos.iter()
            .filter(|info| info.type_id != component_info.type_id)
            .cloned()
            .collect();

        if new_infos.is_empty() {
            // If no components left, despawn the entity
            self.despawn(entity);
            return true;
        }

        let target_arch_idx = self.find_or_create_archetype(new_infos);

        unsafe {
            let archetypes_ptr = self.archetypes.as_mut_ptr();
            let old_arch = &mut *archetypes_ptr.add(arch_idx);
            let new_arch = &mut *archetypes_ptr.add(target_arch_idx);

            new_arch.table.push_entity(entity);

            // Copy all remaining components
            for info in &new_arch.component_infos {
                if let Some(src_ptr) = old_arch.table.get_component_ptr(info.type_id, row) {
                    new_arch.table.push_component_raw(info.type_id, src_ptr);
                }
            }

            // Remove old row and update moved entity location
            if let Some(moved_entity) = old_arch.table.swap_remove(row) {
                self.entity_locations.insert(moved_entity, (arch_idx, row));
            }
        }

        let new_row = self.archetypes[target_arch_idx].table.row_count() - 1;
        self.entity_locations.insert(entity, (target_arch_idx, new_row));
        true
    }

    /// Despawns an entity, removing it from the world entirely.
    pub fn despawn(&mut self, entity: Entity) {
        if let Some((arch_idx, row)) = self.entity_locations.remove(&entity) {
            unsafe {
                let arch = &mut self.archetypes[arch_idx];
                if let Some(moved_entity) = arch.table.swap_remove(row) {
                    self.entity_locations.insert(moved_entity, (arch_idx, row));
                }
            }
            self.entities.deallocate(entity);
        }
    }

    /// Finds or creates an archetype for the given set of component infos.
    fn find_or_create_archetype(&mut self, infos: Vec<ComponentInfo>) -> usize {
        let mut types: Vec<_> = infos.iter().map(|i| i.type_id).collect();
        types.sort();

        if let Some(&id) = self.type_to_archetype.get(&types) {
            id
        } else {
            let id = self.archetypes.len();
            let archetype = Archetype::new(id, infos);
            self.archetypes.push(archetype);
            self.type_to_archetype.insert(types, id);
            id
        }
    }
}

impl Default for World {
    fn default() -> Self { Self::new() }
}