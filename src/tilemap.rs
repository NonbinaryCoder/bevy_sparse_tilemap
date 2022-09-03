//! Storage of tiles and interface with the Bevy engine

use std::{
    mem,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use bevy::{prelude::*, utils::HashMap};

use crate::{tile::Tile, CHUNK_SIZE};

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
    /// Returns a reference to the chunk at the given position if it exists
    #[must_use]
    pub fn get_chunk(&self, pos: IVec2) -> Option<&Chunk<T>> {
        self.data.get(&pos)
    }

    /// Returns a mutable reference to the chunk at the given position if it exists
    #[must_use]
    pub fn get_chunk_mut(&mut self, pos: IVec2) -> Option<&mut Chunk<T>> {
        self.data.get_mut(&pos)
    }

    /// Returns a mutable refernece to the chunk at the given position,
    /// creating one if it doesn't exist
    pub fn get_or_create_chunk(&mut self, pos: IVec2) -> &mut Chunk<T> {
        self.data.entry(pos).or_default()
    }

    /// Returns a reference to the tile at the position in this tilemap if it exists
    #[must_use]
    pub fn get(&self, pos: TilemapPos) -> Option<&T> {
        self.get_chunk(pos.chunk)
            .and_then(|chunk| chunk[pos.tile].as_ref())
    }

    /// Returns a mutable reference to the tile at the position in this tilemap if it exists
    ///
    /// If mutating the tile slot results in a change that requires
    /// regenerating the chunk mesh, call [`regenerate_mesh()`](Chunk::regenerate_mesh())
    /// on the chunk
    #[must_use]
    pub fn get_mut(&mut self, pos: TilemapPos) -> Option<&mut T> {
        self.get_chunk_mut(pos.chunk)
            .and_then(|chunk| chunk[pos.tile].as_mut())
    }

    /// Sets the tile at `pos`, returning it's previous value
    ///
    /// Tells the chunk the tile is in to regenerate it's mesh the next time it's displayed
    pub fn set(&mut self, pos: TilemapPos, tile: impl Into<T>) -> Option<T> {
        self.get_or_create_chunk(pos.chunk)
            .set(pos.tile, tile.into())
    }

    /// Removes the tile at pos and returns it
    ///
    /// Tells the chunk the tile is in to regenerate it's mesh the next time it's displayed
    pub fn remove(&mut self, pos: TilemapPos) -> Option<T> {
        self.get_chunk_mut(pos.chunk)
            .and_then(|chunk| chunk.remove(pos.tile))
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
    fn from(v: IVec2) -> Self {
        TilemapPos {
            chunk: v / IVec2::splat(CHUNK_SIZE as i32),
            tile: ChunkPos::new(
                v.x as u8 & (CHUNK_SIZE as u8 - 1),
                v.y as u8 & (CHUNK_SIZE as u8 - 1),
            ),
        }
    }
}

impl From<TilemapPos> for IVec2 {
    #[must_use]
    #[inline]
    fn from(v: TilemapPos) -> Self {
        v.chunk * (CHUNK_SIZE as i32) + v.tile.to_ivec2()
    }
}

impl Add for TilemapPos {
    type Output = Self;

    #[must_use]
    fn add(self, rhs: Self) -> Self::Output {
        let mut chunk = self.chunk + rhs.chunk;
        let (tile, carry) = self.tile.overflowing_add(rhs.tile);
        if carry.x {
            chunk.x += 1
        }
        if carry.y {
            chunk.y += 1
        }
        TilemapPos { chunk, tile }
    }
}

impl AddAssign for TilemapPos {
    fn add_assign(&mut self, rhs: Self) {
        let (tile, carry) = self.tile.overflowing_add(rhs.tile);
        self.tile = tile;
        self.chunk += rhs.chunk;
        if carry.x {
            self.chunk.x += 1
        }
        if carry.y {
            self.chunk.y += 1
        }
    }
}

impl Sub for TilemapPos {
    type Output = Self;

    #[must_use]
    fn sub(self, rhs: Self) -> Self::Output {
        let mut chunk = self.chunk - rhs.chunk;
        let (tile, carry) = self.tile.overflowing_sub(rhs.tile);
        if carry.x {
            chunk.x -= 1;
        }
        if carry.y {
            chunk.y -= 1;
        }
        TilemapPos { chunk, tile }
    }
}

impl SubAssign for TilemapPos {
    fn sub_assign(&mut self, rhs: Self) {
        let (tile, carry) = self.tile.overflowing_sub(rhs.tile);
        self.tile = tile;
        self.chunk -= rhs.chunk;
        if carry.x {
            self.chunk.x -= 1
        }
        if carry.y {
            self.chunk.y -= 1
        }
    }
}

impl Add<ChunkPos> for TilemapPos {
    type Output = Self;

    #[must_use]
    fn add(self, rhs: ChunkPos) -> Self::Output {
        let mut chunk = self.chunk;
        let (tile, carry) = self.tile.overflowing_add(rhs);
        if carry.x {
            chunk.x += 1
        }
        if carry.y {
            chunk.y += 1
        }
        TilemapPos { chunk, tile }
    }
}

impl AddAssign<ChunkPos> for TilemapPos {
    fn add_assign(&mut self, rhs: ChunkPos) {
        let (tile, carry) = self.tile.overflowing_add(rhs);
        self.tile = tile;
        if carry.x {
            self.chunk.x += 1
        }
        if carry.y {
            self.chunk.y += 1
        }
    }
}

impl Sub<ChunkPos> for TilemapPos {
    type Output = Self;

    #[must_use]
    fn sub(self, rhs: ChunkPos) -> Self::Output {
        let mut chunk = self.chunk;
        let (tile, carry) = self.tile.overflowing_sub(rhs);
        if carry.x {
            chunk.x -= 1
        }
        if carry.y {
            chunk.y -= 1
        }
        TilemapPos { chunk, tile }
    }
}

impl SubAssign<ChunkPos> for TilemapPos {
    fn sub_assign(&mut self, rhs: ChunkPos) {
        let (tile, carry) = self.tile.overflowing_sub(rhs);
        self.tile = tile;
        if carry.x {
            self.chunk.x -= 1
        }
        if carry.y {
            self.chunk.y -= 1
        }
    }
}

impl Add<IVec2> for TilemapPos {
    type Output = Self;

    /// `self` + `rhs` chunks
    fn add(self, rhs: IVec2) -> Self::Output {
        TilemapPos {
            chunk: self.chunk + rhs,
            tile: self.tile,
        }
    }
}

impl AddAssign<IVec2> for TilemapPos {
    /// `self` += `rhs` chunks
    fn add_assign(&mut self, rhs: IVec2) {
        self.chunk += rhs
    }
}

impl Sub<IVec2> for TilemapPos {
    type Output = Self;

    /// `self` - `rhs` chunks
    fn sub(self, rhs: IVec2) -> Self::Output {
        TilemapPos {
            chunk: self.chunk - rhs,
            tile: self.tile,
        }
    }
}

impl SubAssign<IVec2> for TilemapPos {
    /// `self` -= `rhs` chunks
    fn sub_assign(&mut self, rhs: IVec2) {
        self.chunk -= rhs
    }
}
