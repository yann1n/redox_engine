//! Conversions between `glam` types and common external representations.
//!
//! This module provides extension traits that allow converting RedOx Math types
//! to and from arrays, tuples, and other standard layouts. These conversions are
//! useful when passing data to graphics APIs (e.g., `wgpu`), physics engines
//! (e.g., `rapier3d`), or serialization formats.
//!
//! # Examples
//! ```
//! use redox_math::{Vec3, Mat4, conversions::*};
//!
//! let v = Vec3::new(1.0, 2.0, 3.0);
//! let arr = v.to_array();
//! assert_eq!(arr, [1.0, 2.0, 3.0]);
//!
//! let m = Mat4::IDENTITY;
//! let cols = m.to_cols_array_2d();
//! assert_eq!(cols[0][0], 1.0);
//! ```

use crate::*;

/// Extension trait for [`Vec2`] providing array and tuple conversions.
pub trait Vec2Ext {
    /// Converts to `[f32; 2]`.
    fn to_array(&self) -> [f32; 2];
    /// Converts to `(f32, f32)`.
    fn to_tuple(&self) -> (f32, f32);
    /// Constructs from `[f32; 2]`.
    fn from_array(arr: [f32; 2]) -> Self;
    /// Constructs from `(f32, f32)`.
    fn from_tuple(tup: (f32, f32)) -> Self;
}

impl Vec2Ext for Vec2 {
    #[inline]
    fn to_array(&self) -> [f32; 2] {
        (*self).into()
    }

    #[inline]
    fn to_tuple(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    #[inline]
    fn from_array(arr: [f32; 2]) -> Self {
        arr.into()
    }

    #[inline]
    fn from_tuple(tup: (f32, f32)) -> Self {
        tup.into()
    }
}

/// Extension trait for [`Vec3`] providing array and tuple conversions.
pub trait Vec3Ext {
    /// Converts to `[f32; 3]`.
    fn to_array(&self) -> [f32; 3];
    /// Converts to `(f32, f32, f32)`.
    fn to_tuple(&self) -> (f32, f32, f32);
    /// Constructs from `[f32; 3]`.
    fn from_array(arr: [f32; 3]) -> Self;
    /// Constructs from `(f32, f32, f32)`.
    fn from_tuple(tup: (f32, f32, f32)) -> Self;
}

impl Vec3Ext for Vec3 {
    #[inline]
    fn to_array(&self) -> [f32; 3] {
        (*self).into()
    }

    #[inline]
    fn to_tuple(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }

    #[inline]
    fn from_array(arr: [f32; 3]) -> Self {
        arr.into()
    }

    #[inline]
    fn from_tuple(tup: (f32, f32, f32)) -> Self {
        tup.into()
    }
}

/// Extension trait for [`Vec4`] providing array and tuple conversions.
pub trait Vec4Ext {
    /// Converts to `[f32; 4]`.
    fn to_array(&self) -> [f32; 4];
    /// Converts to `(f32, f32, f32, f32)`.
    fn to_tuple(&self) -> (f32, f32, f32, f32);
    /// Constructs from `[f32; 4]`.
    fn from_array(arr: [f32; 4]) -> Self;
    /// Constructs from `(f32, f32, f32, f32)`.
    fn from_tuple(tup: (f32, f32, f32, f32)) -> Self;
}

impl Vec4Ext for Vec4 {
    #[inline]
    fn to_array(&self) -> [f32; 4] {
        (*self).into()
    }

    #[inline]
    fn to_tuple(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.z, self.w)
    }

    #[inline]
    fn from_array(arr: [f32; 4]) -> Self {
        arr.into()
    }

    #[inline]
    fn from_tuple(tup: (f32, f32, f32, f32)) -> Self {
        tup.into()
    }
}

/// Extension trait for [`Mat4`] providing column‑major array conversions.
///
/// These conversions are particularly useful when uploading matrices to GPU
/// uniform buffers (e.g., with `wgpu`), because graphics APIs expect matrices
/// in column‑major order.
pub trait Mat4Ext {
    /// Returns the matrix as a column‑major `[[f32; 4]; 4]`.
    ///
    /// This is the layout typically required for uniform buffers in Vulkan,
    /// Direct3D 12, Metal, and OpenGL. Each inner array represents a column.
    fn to_cols_array_2d(&self) -> [[f32; 4]; 4];

    /// Returns the matrix as a flat `[f32; 16]` in column‑major order.
    ///
    /// This is equivalent to flattening `to_cols_array_2d()`.
    fn to_cols_array(&self) -> [f32; 16];

    /// Constructs a matrix from a column‑major `[[f32; 4]; 4]`.
    fn from_cols_array_2d(arr: [[f32; 4]; 4]) -> Self;

    /// Constructs a matrix from a flat `[f32; 16]` in column‑major order.
    fn from_cols_array(arr: [f32; 16]) -> Self;
}

impl Mat4Ext for Mat4 {
    #[inline]
    fn to_cols_array_2d(&self) -> [[f32; 4]; 4] {
        self.to_cols_array_2d()
    }

    #[inline]
    fn to_cols_array(&self) -> [f32; 16] {
        self.to_cols_array()
    }

    #[inline]
    fn from_cols_array_2d(arr: [[f32; 4]; 4]) -> Self {
        Self::from_cols_array_2d(&arr)
    }

    #[inline]
    fn from_cols_array(arr: [f32; 16]) -> Self {
        Self::from_cols_array(&arr)
    }
}

/// Extension trait for [`Quat`] providing conversions to/from arrays and tuples.
pub trait QuatExt {
    /// Converts to `[f32; 4]` (x, y, z, w).
    fn to_array(&self) -> [f32; 4];
    /// Converts to `(f32, f32, f32, f32)`.
    fn to_tuple(&self) -> (f32, f32, f32, f32);
    /// Constructs from `[f32; 4]` (x, y, z, w).
    fn from_array(arr: [f32; 4]) -> Self;
    /// Constructs from `(f32, f32, f32, f32)`.
    fn from_tuple(tup: (f32, f32, f32, f32)) -> Self;
}

impl QuatExt for Quat {
    #[inline]
    fn to_array(&self) -> [f32; 4] {
        (*self).into()
    }

    #[inline]
    fn to_tuple(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.z, self.w)
    }

    #[inline]
    fn from_array(arr: [f32; 4]) -> Self {
        Self::from_xyzw(arr[0], arr[1], arr[2], arr[3])
    }

    #[inline]
    fn from_tuple(tup: (f32, f32, f32, f32)) -> Self {
        Self::from_xyzw(tup.0, tup.1, tup.2, tup.3)
    }
}