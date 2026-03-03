use std::marker::PhantomData;
use crate::component::Component;

/// Filter that ensures a component exists.
pub struct With<T: Component>(PhantomData<T>);

/// Filter that ensures a component does **not** exist.
pub struct Without<T: Component>(PhantomData<T>);

/// Filter for components that were modified in the current or previous frame.
///
/// **Note:** Not yet implemented; placeholder for future extension.
pub struct Changed<T: Component>(PhantomData<T>);

/// Filter for entities created in the current frame.
///
/// **Note:** Not yet implemented; placeholder for future extension.
pub struct Added<T: Component>(PhantomData<T>);