//! Creation of meshes from tilemaps

use std::fmt::Debug;

use bevy::{prelude::*, sprite::Material2d};

/// Trait for types used to build meshes for tilemap [`Chunk`](crate::tilemap::Chunk)s
pub trait MeshBuilder {
    /// Saved after mesh generation is finished and
    /// passed back when mesh generation begins again
    ///
    /// Used to store information that can be used to speed up mesh generation
    /// (for example, the number of vertices last used).  If the mesh for the chunk
    /// this is generating has not been generated recently, the default value of
    /// `CarryData` is used
    type CarryData: Debug + Default + Send + Sync;

    /// The type of the material meshes from this use
    type Material: Material2d;

    /// Returns the material that meshes from this use
    ///
    /// With the current implementation this function is only ever called once
    /// and cached, but this may change in the future
    fn material() -> Self::Material;

    /// Creates a new instance of this that will be fed every tile in the chunk for
    /// mesh generation
    fn init(carry_data: Self::CarryData) -> Self;

    /// Sets the offset at which to add the next few tile meshes
    fn set_offset(&mut self, offset: Vec2);

    /// Finishes mesh generation.
    ///
    /// Returns the generated mesh and the new carry data
    fn finish(self) -> (Mesh, Self::CarryData);
}
