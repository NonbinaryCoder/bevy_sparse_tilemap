//! Storage of tiles and interface with the Bevy engine

use std::ops::{Add, AddAssign, Sub, SubAssign};

use bevy::{prelude::*, utils::HashMap};

use crate::tile::Tile;

mod chunk;

pub use chunk::*;

/// A tilemap with tiles of type `T`
///
/// Uses chunks to generate more efficient meshes
#[derive(Debug)]
pub struct Tilemap<T: Tile> {
    data: HashMap<IVec2, Chunk<T>>,
}

impl<T: Tile> Tilemap<T> {
    /// Returns a reference to the tile at the position in this tilemap if it exists
    #[must_use]
    pub fn get(&self, pos: TilemapPos) -> Option<&T> {
        todo!()
    }

    /// Returns a mutable reference to the tile at the position in this tilemap if it exists
    ///
    /// Useful for in-place manipulation
    #[must_use]
    pub fn get_mut(&mut self, pos: TilemapPos) -> Option<&mut T> {
        todo!()
    }

    /// Returns a mutable referenct to the tile slot at the position,
    /// allocating one if it doesn't exist
    ///
    /// If you know there will be a tile at the position, use [`Self::get_mut`] instead
    #[must_use]
    pub fn get_or_alloc_mut(&mut self, pos: TilemapPos) -> &mut Option<T> {
        todo!()
    }

    /// Removes the tile at pos
    pub fn remove(&mut self, pos: TilemapPos) -> Option<T> {
        todo!()
    }

    /// Inserts a tile into the tilemap
    ///
    /// Returns the replaced tile if one exists
    pub fn insert(&mut self, tile: impl Into<T>, pos: TilemapPos) -> Option<T> {
        todo!()
    }
}

/// A position in a tilemap
///
/// Stored as the chunk the position is in and which tile the position is
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TilemapPos {
    /// The chunk the position is in
    pub chunk: IVec2,
    /// The tile in the chunk the position is
    pub tile: ChunkPos,
}

impl From<IVec2> for TilemapPos {
    #[must_use]
    fn from(_: IVec2) -> Self {
        todo!()
    }
}

impl From<TilemapPos> for IVec2 {
    #[must_use]
    fn from(_: TilemapPos) -> Self {
        todo!()
    }
}

impl Add for TilemapPos {
    type Output = Self;

    #[must_use]
    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl AddAssign for TilemapPos {
    fn add_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl Sub for TilemapPos {
    type Output = Self;

    #[must_use]
    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl SubAssign for TilemapPos {
    fn sub_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl Add<ChunkPos> for TilemapPos {
    type Output = Self;

    #[must_use]
    fn add(self, rhs: ChunkPos) -> Self::Output {
        todo!()
    }
}

impl AddAssign<ChunkPos> for TilemapPos {
    fn add_assign(&mut self, rhs: ChunkPos) {
        todo!()
    }
}

impl Sub<ChunkPos> for TilemapPos {
    type Output = Self;

    #[must_use]
    fn sub(self, rhs: ChunkPos) -> Self::Output {
        todo!()
    }
}

impl SubAssign<ChunkPos> for TilemapPos {
    fn sub_assign(&mut self, rhs: ChunkPos) {
        todo!()
    }
}
