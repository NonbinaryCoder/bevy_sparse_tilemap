//! Creation of meshes from tilemaps

use std::fmt::Debug;

/// Trait for types used to build meshes for tilemap [`Chunk`](super::tilemap::Chunk)s
pub trait MeshBuilder {
    /// Saved after mesh generation is finished and
    /// passed back when mesh generation begins again
    ///
    /// Used to store information that can be used to speed up mesh generation.
    /// For example, the number of vertices last used
    ///
    /// If the mesh for the chunk this is generating has not been generated recently,
    /// the default value of `CarryData` is used
    type CarryData: Debug + Default + Send + Sync;
}
