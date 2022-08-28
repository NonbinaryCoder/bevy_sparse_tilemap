use std::ops::{Add, AddAssign, Sub, SubAssign};

use bevy::prelude::*;

use crate::{rendering::MeshBuilder, tile::Tile, CHUNK_SIZE};

/// A chunk of tiles;
/// [`CHUNK_SIZE`](super::super::CHUNK_SIZE) by [`CHUNK_SIZE`](super::super::CHUNK_SIZE)
#[derive(Debug)]
pub struct Chunk<T: Tile> {
    _tiles: [Option<T>; CHUNK_SIZE * CHUNK_SIZE],
    _mesh_carry_data: <<T as Tile>::MeshBuilder as MeshBuilder>::CarryData,
    _mesh_entity: Entity,
}

impl<T: Tile> Chunk<T> {
    /// Returns a reference to the tile at the position in this chunk
    #[must_use]
    pub fn get(&self, pos: ChunkPos) -> &Option<T> {
        todo!()
    }

    /// Returns a mutable referene to the tile at the position in this chunk
    #[must_use]
    pub fn get_mut(&mut self, pos: ChunkPos) -> &mut Option<T> {
        todo!()
    }
}

/// A position in a chunk
///
/// Gaurenteed to be between 0 and [`CHUNK_SIZE`](super::super::CHUNK_SIZE)
///
/// # Notes
///
/// Adding and subtracting chunk pos panics on overflow.
/// If you want to carry, convert one to a [`TilemapPos`](super::TilemapPos) first
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkPos(u8, u8);

impl ChunkPos {
    /// Creates a [`ChunkPos`] with the given x and y coordinates
    ///
    /// # Panics
    ///
    /// Panics if either coordinate is not in the range
    /// 0 <= x < [`CHUNK_SIZE`](super::super::CHUNK_SIZE)
    #[must_use]
    #[inline]
    pub fn new(x: u8, y: u8) -> Self {
        todo!()
    }

    /// The x coordinate of this
    #[must_use]
    #[inline]
    pub fn x(self) -> u8 {
        self.0
    }

    /// The y coordinate of this
    #[must_use]
    #[inline]
    pub fn y(self) -> u8 {
        self.1
    }

    /// Sets the x position of this
    ///
    /// # Panics
    ///
    /// Panics if x >= [`CHUNK_SIZE`](super::super::CHUNK_SIZE)
    #[inline]
    pub fn set_x(&mut self, x: u8) {
        todo!()
    }

    /// Sets the y position of this
    ///
    /// # Panics
    ///
    /// Panics if y >= [`CHUNK_SIZE`](super::super::CHUNK_SIZE)
    #[inline]
    pub fn set_y(&mut self, y: u8) {
        todo!()
    }
}

impl TryFrom<IVec2> for ChunkPos {
    type Error = ToChunkPosError;

    fn try_from(value: IVec2) -> Result<Self, Self::Error> {
        todo!()
    }
}

/// The error returned from failing to convert an [`IVec2`] to a [`ChunkPos`]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToChunkPosError {
    /// One of the coordinates was negative
    Negative,
    /// One of the coordinates was >= [`CHUNK_SIZE`](super::super::CHUNK_SIZE)
    TooBig,
}

impl Add for ChunkPos {
    type Output = Self;

    #[must_use]
    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl AddAssign for ChunkPos {
    fn add_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl Sub for ChunkPos {
    type Output = Self;

    #[must_use]
    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl SubAssign for ChunkPos {
    fn sub_assign(&mut self, rhs: Self) {
        todo!()
    }
}
