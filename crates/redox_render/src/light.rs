//! Directional light component and GPU uniform.

use bytemuck::{Pod, Zeroable};
use redox_math::Vec3;

/// ECS component for a directional (sun-like) light source.
///
/// In the MVP only one directional light is supported. The `direction` vector
/// points *towards* the light — i.e., opposite to the direction the light
/// rays travel.
#[derive(Clone, Debug)]
pub struct DirectionalLight {
    /// Normalised direction **towards** the light source.
    pub direction: Vec3,
    /// Light colour (linear RGB, not sRGB).
    pub color: Vec3,
    /// Intensity multiplier.
    pub intensity: f32,
}

impl DirectionalLight {
    /// Creates a new directional light.
    pub fn new(direction: Vec3, color: Vec3, intensity: f32) -> Self {
        Self {
            direction: direction.normalize(),
            color,
            intensity,
        }
    }
}

impl Default for DirectionalLight {
    /// Default: white light coming from above and slightly to the side.
    fn default() -> Self {
        Self::new(Vec3::new(0.3, 1.0, 0.5), Vec3::ONE, 1.0)
    }
}

/// GPU-friendly light parameters.
///
/// Bound as a uniform buffer in the fragment shader.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct LightUniform {
    /// Direction towards the light (xyz) + padding (w).
    pub direction: [f32; 4],
    /// Colour × intensity (xyz) + padding (w).
    pub color: [f32; 4],
    /// Ambient colour (xyz) + padding (w).
    pub ambient: [f32; 4],
}

impl LightUniform {
    /// Builds the uniform from a [`DirectionalLight`] and an ambient term.
    pub fn from_light(light: &DirectionalLight, ambient: Vec3) -> Self {
        let c = light.color * light.intensity;
        Self {
            direction: [light.direction.x, light.direction.y, light.direction.z, 0.0],
            color: [c.x, c.y, c.z, 1.0],
            ambient: [ambient.x, ambient.y, ambient.z, 1.0],
        }
    }
}

impl Default for LightUniform {
    fn default() -> Self {
        Self::from_light(&DirectionalLight::default(), Vec3::splat(0.15))
    }
}
