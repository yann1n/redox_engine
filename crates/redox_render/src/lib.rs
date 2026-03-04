//! Rendering subsystem for the RedOx Engine.
//!
//! This crate provides a `wgpu`-based renderer with forward shading,
//! procedural mesh generation, texture loading, and ECS integration
//! through the `redox_ecs` crate.

pub mod camera;
pub mod context;
pub mod light;
pub mod material;
pub mod mesh;
pub mod pass;
pub mod resource;
pub mod shader;
pub mod systems;

pub use camera::{ActiveCamera, Camera, CameraUniform};
pub use context::RenderContext;
pub use light::{DirectionalLight, LightUniform};
pub use material::Material;
pub use mesh::{Mesh, Vertex};
pub use systems::{MaterialHandle, MeshHandle, RenderObject, Transform};
