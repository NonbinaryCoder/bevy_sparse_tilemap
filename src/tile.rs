//! Tiles in the tilemap

use crate::{animation::MeshUpdater, rendering::MeshBuilder};

/// A tile in the tilemap
pub trait Tile: Send + Sync + Clone + 'static {
    /// The struct this uses to add it's mesh to the chunk mesh
    type MeshBuilder: MeshBuilder;
    /// The struct this uses to apply animations to the chunk mesh
    ///
    /// If not using animations, this can be set to `()`
    type MeshUpdater: MeshUpdater;

    /// Add this tile to the mesh
    ///
    /// Called before [`Self::animate`]
    fn add_to_mesh(&self, builder: &mut Self::MeshBuilder);

    /// Animates this tile
    ///
    /// If animation is enabled, called after [`Self::add_to_mesh`].
    /// Otherwise, never called.  Default implementation is to panic;
    /// if using animation, this method must be implemented
    #[allow(unused_variables)]
    fn animate(&mut self, updater: &mut Self::MeshUpdater, delta_time: f32) {
        unimplemented!()
    }
}
