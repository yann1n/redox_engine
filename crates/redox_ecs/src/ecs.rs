//! The core ECS (Entity-Component System) module for RedOx Engine.
//!
//! This crate provides a high-performance archetype-based ECS implementation
//! with support for queries, events, hierarchies, and parallel iteration.

pub mod entity;
pub mod component;
pub mod archetype;
pub mod world;
pub mod query;
pub mod event;
pub mod system;
pub mod hierarchy;

pub use entity::{Entity, EntityAllocator};
pub use component::Component;
pub use world::World;
pub use query::{Query, ParallelQuery};
pub use event::{Events, EventReader};
pub use system::{System, SystemStage};
pub use hierarchy::{Parent, Children};

/// The current version of the `redox_ecs` crate.
pub const REDOX_ECS_VERSION: &str = env!("CARGO_PKG_VERSION");