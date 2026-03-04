//! Material description for surface appearance.

use redox_math::Vec3;

/// Describes the visual properties of a surface.
///
/// In the MVP only `base_color` and an optional texture are used.
/// `metallic` and `roughness` are stored for future PBR expansion.
#[derive(Clone, Debug)]
pub struct Material {
    /// Base albedo colour (linear RGB).
    pub base_color: Vec3,
    /// Optional index into the texture storage.
    /// `None` means the material uses only the solid `base_color`.
    pub texture_index: Option<usize>,
    /// Metallic factor (0.0 = dielectric, 1.0 = metal). PBR stub.
    pub metallic: f32,
    /// Roughness factor (0.0 = mirror, 1.0 = rough). PBR stub.
    pub roughness: f32,
}

impl Material {
    /// Creates a simple solid-colour material.
    pub fn solid(color: Vec3) -> Self {
        Self {
            base_color: color,
            texture_index: None,
            metallic: 0.0,
            roughness: 0.5,
        }
    }

    /// Creates a textured material.
    pub fn textured(color: Vec3, texture_index: usize) -> Self {
        Self {
            base_color: color,
            texture_index: Some(texture_index),
            metallic: 0.0,
            roughness: 0.5,
        }
    }
}

impl Default for Material {
    /// Default: opaque white, no texture.
    fn default() -> Self {
        Self::solid(Vec3::ONE)
    }
}
