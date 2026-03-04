//! ECS components and systems for render integration.
//!
//! Defines the `Transform`, `MeshHandle`, and `MaterialHandle` components
//! and the per-frame `RenderObject` struct collected by `extract_render_data`.

use redox_math::{Mat4, Quat, Vec3, transform_matrix};

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

/// Spatial transform component (translation, rotation, uniform/non-uniform scale).
#[derive(Clone, Debug)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    /// Identity transform (origin, no rotation, unit scale).
    pub const IDENTITY: Self = Self {
        translation: Vec3::ZERO,
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    };

    /// Creates a transform with only a translation.
    pub fn from_translation(t: Vec3) -> Self {
        Self {
            translation: t,
            ..Self::IDENTITY
        }
    }

    /// Creates a transform from translation and rotation.
    pub fn from_translation_rotation(t: Vec3, r: Quat) -> Self {
        Self {
            translation: t,
            rotation: r,
            ..Self::IDENTITY
        }
    }

    /// Computes the 4×4 model matrix (`T * R * S`).
    #[inline]
    pub fn matrix(&self) -> Mat4 {
        transform_matrix(self.translation, self.rotation, self.scale)
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::IDENTITY
    }
}

/// Index into the mesh storage inside [`RenderContext`](crate::context::RenderContext).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MeshHandle(pub usize);

/// Index into the material storage inside [`RenderContext`](crate::context::RenderContext).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MaterialHandle(pub usize);

// ---------------------------------------------------------------------------
// Per-frame render data
// ---------------------------------------------------------------------------

/// Temporary structure built each frame during the `RenderPrep` stage.
///
/// One `RenderObject` per visible entity. Consumed by `ForwardPass::render`.
#[derive(Clone, Debug)]
pub struct RenderObject {
    /// Model matrix for this entity.
    pub model_matrix: Mat4,
    /// Index of the mesh in the render context.
    pub mesh_index: usize,
    /// Index of the material in the render context.
    pub material_index: usize,
}
