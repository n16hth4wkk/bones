//! Transform component.

use crate::prelude::*;

/// The main transform component.
///
/// Currently we don't have a hierarchy, and this is therefore a global transform.
#[derive(Clone, Copy, Debug, TypeUlid)]
#[ulid = "01GNFQWJWWXJJEXZQEDQJWZQWP"]
#[repr(C)]
pub struct Transform {
    /// The position of the entity in the world.
    pub translation: Vec3,
    /// The rotation of the entity.
    pub rotation: Quat,
    /// The scale of the entity.
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Default::default(),
            rotation: Default::default(),
            scale: Vec3::ONE,
        }
    }
}