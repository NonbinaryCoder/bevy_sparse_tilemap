use std::{
    iter, mem,
    ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign},
};

use bevy::prelude::*;

use crate::{rendering::MeshBuilder, tile::Tile, CHUNK_SIZE};

/// A chunk of tiles; [`CHUNK_SIZE`] by [`CHUNK_SIZE`]
#[derive(Debug)]
pub struct Chunk<T: Tile> {
    tiles: [Option<T>; CHUNK_SIZE * CHUNK_SIZE],
    pub(crate) regenerate_mesh: bool,
    pub(crate) mesh_carry_data: <<T as Tile>::MeshBuilder as MeshBuilder>::CarryData,
    pub(crate) mesh_entity: Option<Entity>,
}

impl<T: Tile> Chunk<T> {
    /// Returns `true` if there is a tile at the position specified
    #[must_use]
    pub fn is_set(&self, pos: ChunkPos) -> bool {
        self[pos].is_some()
    }

    /// Tells this to regenerate it's mesh the next time it is displayed
    ///
    /// Mesh regeneration is more expensive than animation, so use animation whenever
    /// possible
    #[inline]
    pub fn regenerate_mesh(&mut self) {
        self.regenerate_mesh = true;
    }

    /// Sets the tile at `pos`, returning it's previous value
    ///
    /// Tells this to regenerate it's mesh the next time it is displayed
    pub fn set(&mut self, pos: ChunkPos, tile: impl Into<T>) -> Option<T> {
        self.regenerate_mesh();
        mem::replace(&mut self[pos], Some(tile.into()))
    }

    /// Removes the tile at `pos`, returning it's previous value
    ///
    /// Tells this to regenerate it's mesh the next time it is displayed
    pub fn remove(&mut self, pos: ChunkPos) -> Option<T> {
        self.regenerate_mesh();
        mem::take(&mut self[pos])
    }

    /// Returns an iterator over all tile slots in this
    ///
    /// Iterates in row-major order
    pub fn iter(&self) -> impl Iterator<Item = &Option<T>> + ExactSizeIterator {
        self.tiles.iter()
    }

    /// Returns an iterator over all tile slots in this that allows modifying each value
    ///
    /// Iterates in row-major order.
    /// If mutating the tile slot results in a change that requires
    /// regenerating the chunk mesh, call [`Self::regenerate_mesh()`]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Option<T>> + ExactSizeIterator {
        self.tiles.iter_mut()
    }

    /// Returns an iterator over all the set tiles in this
    pub fn iter_tiles(&self) -> impl Iterator<Item = &T> {
        self.iter().filter_map(|t| t.as_ref())
    }

    /// Returns an iterator over all the set tiles in this that allows modifying each value
    ///
    /// If mutating the tile slot results in a change that requires
    /// regenerating the chunk mesh, call [`Self::regenerate_mesh()`]
    pub fn iter_tiles_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.iter_mut().filter_map(|t| t.as_mut())
    }

    /// Returns an iterator over all tile slots in this and their position
    ///
    /// Iterates in row-major order
    ///
    /// If you are looking for an iterator over all positions in a chunk,
    /// use [`ChunkPos::iter_positions()`]
    pub fn iter_positions(&self) -> impl Iterator<Item = (ChunkPos, &Option<T>)> {
        ChunkPos::iter_positions().zip(self.iter())
    }

    /// Returns an iterator over all tile slots in this and their position
    /// that allows modifying each tile slot
    ///
    /// Iterates in row-major order.
    /// If mutating the tile slot results in a change that requires
    /// regenerating the chunk mesh, call [`Self::regenerate_mesh()`]
    ///
    /// If you are looking for an iterator over all positions in a chunk,
    /// use [`ChunkPos::iter_positions()`]
    pub fn iter_positions_mut(&mut self) -> impl Iterator<Item = (ChunkPos, &mut Option<T>)> {
        ChunkPos::iter_positions().zip(self.iter_mut())
    }

    /// Returns an iterator over all the set tiles in this and their positions
    pub fn iter_tile_positions(&self) -> impl Iterator<Item = (ChunkPos, &T)> {
        self.iter_positions()
            .filter_map(|(pos, slot)| slot.as_ref().map(|tile| (pos, tile)))
    }

    /// Returns an iterator over all the set tiles in this and their positions
    /// that allows mutating each tile
    ///
    /// If mutating the tile slot results in a change that requires
    /// regenerating the chunk mesh, call [`Self::regenerate_mesh()`]
    pub fn iter_tile_positions_mut(&mut self) -> impl Iterator<Item = (ChunkPos, &mut T)> {
        self.iter_positions_mut()
            .filter_map(|(pos, slot)| slot.as_mut().map(|tile| (pos, tile)))
    }
}

impl<T: Tile> Default for Chunk<T> {
    fn default() -> Self {
        Chunk {
            tiles: [(); CHUNK_SIZE * CHUNK_SIZE].map(|_| None),
            regenerate_mesh: false,
            mesh_carry_data: <<T as Tile>::MeshBuilder as MeshBuilder>::CarryData::default(),
            mesh_entity: None,
        }
    }
}

impl<T: Tile> Index<ChunkPos> for Chunk<T> {
    type Output = Option<T>;

    /// Returns a reference to the tile slot at the index
    #[must_use]
    fn index(&self, index: ChunkPos) -> &Self::Output {
        &self.tiles[index.as_index()]
    }
}

impl<T: Tile> IndexMut<ChunkPos> for Chunk<T> {
    /// Returns a mutable reference to the tile slot at the index
    ///
    /// If mutating the tile slot results in a change that requires
    /// regenerating the chunk mesh, call [`Self::regenerate_mesh()`]
    #[must_use]
    fn index_mut(&mut self, index: ChunkPos) -> &mut Self::Output {
        &mut self.tiles[index.as_index()]
    }
}

/// A position in a chunk
///
/// Gaurenteed to be between 0 and [`CHUNK_SIZE`]
///
/// # Notes
///
/// Adding and subtracting chunk pos panics on overflow.
/// If you want to carry, convert one to a [`TilemapPos`](super::TilemapPos) first.
/// If you want to wrap around chunk borders, use
/// [`wrapping_add`](Self::wrapping_add) or [`wrapping_sub`](Self::wrapping_add)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPos(u8, u8);

impl ChunkPos {
    /// The position of the tile at (0, 0) in a chunk
    pub const ZERO: Self = ChunkPos(0, 0);

    /// Creates a [`ChunkPos`] with the given x and y coordinates
    ///
    /// # Panics
    ///
    /// Panics if x or y is >= [`CHUNK_SIZE`]
    #[must_use]
    #[inline]
    pub fn new(x: u8, y: u8) -> Self {
        Self::try_new(x, y).unwrap()
    }

    /// Returns [`Some`] if a [`ChunkPos`] could be constructed from the given coordinates
    #[must_use]
    #[inline]
    pub fn try_new(x: u8, y: u8) -> Option<Self> {
        if x < CHUNK_SIZE as u8 && y < CHUNK_SIZE as u8 {
            Some(ChunkPos(x, y))
        } else {
            None
        }
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

    /// Returns a tuple of the x and y position of this
    #[must_use]
    #[inline]
    pub fn tup(self) -> (u8, u8) {
        (self.x(), self.y())
    }

    /// Sets the x position of this
    ///
    /// # Panics
    ///
    /// Panics if x >= [`CHUNK_SIZE`]
    #[inline]
    pub fn set_x(&mut self, x: u8) {
        assert!(x <= CHUNK_SIZE as u8);
        self.0 = x
    }

    /// Sets the y position of this
    ///
    /// # Panics
    ///
    /// Panics if y >= [`CHUNK_SIZE`]
    #[inline]
    pub fn set_y(&mut self, y: u8) {
        assert!(y <= CHUNK_SIZE as u8);
        self.0 = y
    }

    /// This, but as an index into a [`CHUNK_SIZE`]x[`CHUNK_SIZE`] array (row-major)
    #[must_use]
    pub fn as_index(self) -> usize {
        let ChunkPos(x, y) = self;
        x as usize + y as usize * CHUNK_SIZE
    }

    /// This as an [`IVec2`]
    #[must_use]
    pub fn as_ivec2(self) -> IVec2 {
        self.into()
    }

    /// `self` + `rhs`, wrapped at [`CHUNK_SIZE`]
    #[must_use]
    pub fn wrapping_add(self, rhs: Self) -> Self {
        ChunkPos(
            self.x().wrapping_add(rhs.x()) & (CHUNK_SIZE as u8 - 1),
            self.y().wrapping_add(rhs.y()) & (CHUNK_SIZE as u8 - 1),
        )
    }

    /// `self` - `rhs`, wrapped at [`CHUNK_SIZE`]
    #[must_use]
    pub fn wrapping_sub(self, rhs: Self) -> Self {
        ChunkPos(
            self.x().wrapping_sub(rhs.x()) & (CHUNK_SIZE as u8 - 1),
            self.y().wrapping_sub(rhs.y()) & (CHUNK_SIZE as u8 - 1),
        )
    }

    /// `self` + `rhs`
    ///
    /// Returns the result of the addition wrapped around [`CHUNK_SIZE`],
    /// and a [`BVec2`] indicating whether overflow occured
    #[must_use]
    pub fn overflowing_add(self, rhs: Self) -> (Self, BVec2) {
        fn overflowing_add_u8(lhs: u8, rhs: u8) -> (u8, bool) {
            let r = lhs.wrapping_add(rhs);
            (r & (CHUNK_SIZE as u8 - 1), r > CHUNK_SIZE as u8)
        }
        let (x, cx) = overflowing_add_u8(self.x(), rhs.x());
        let (y, cy) = overflowing_add_u8(self.y(), rhs.y());
        (ChunkPos(x, y), BVec2::new(cx, cy))
    }

    /// `self` - `rhs`
    ///
    /// Returns the result of the subtraction wrapped around [`CHUNK_SIZE`],
    /// and a [`BVec2`] indicating whether overflow occured
    pub fn overflowing_sub(self, rhs: Self) -> (Self, BVec2) {
        fn overflowing_sub_u8(lhs: u8, rhs: u8) -> (u8, bool) {
            let (sum, carry) = lhs.overflowing_sub(rhs);
            (sum & (CHUNK_SIZE as u8 - 1), carry)
        }
        let (x, cx) = overflowing_sub_u8(self.x(), rhs.x());
        let (y, cy) = overflowing_sub_u8(self.y(), rhs.y());
        (ChunkPos(x, y), BVec2::new(cx, cy))
    }

    /// Returns an iterator over all possible positions
    ///
    /// Iterates in row-major order
    pub fn iter_positions() -> impl Iterator<Item = Self> {
        iter::successors(Some(ChunkPos::ZERO), |pos| {
            let (mut x, mut y) = pos.tup();
            x += 1;
            if x == CHUNK_SIZE as u8 {
                x = 0;
                y += 1;
            }
            ChunkPos::try_new(x, y)
        })
    }
}

impl TryFrom<IVec2> for ChunkPos {
    type Error = ();

    fn try_from(value: IVec2) -> Result<Self, Self::Error> {
        match value.x >= 0
            && value.x < CHUNK_SIZE as i32
            && value.y >= 0
            && value.y < CHUNK_SIZE as i32
        {
            true => Ok(ChunkPos(value.x as u8, value.y as u8)),
            false => Err(()),
        }
    }
}

impl From<ChunkPos> for IVec2 {
    #[must_use]
    #[inline]
    fn from(v: ChunkPos) -> Self {
        IVec2::new(v.0 as i32, v.1 as i32)
    }
}

impl Add for ChunkPos {
    type Output = Self;

    #[must_use]
    fn add(self, rhs: Self) -> Self::Output {
        ChunkPos::new(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for ChunkPos {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        assert!(self.0 < CHUNK_SIZE as u8 && self.1 < CHUNK_SIZE as u8);
    }
}

impl Sub for ChunkPos {
    type Output = Self;

    #[must_use]
    fn sub(self, rhs: Self) -> Self::Output {
        ChunkPos::new(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign for ChunkPos {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        assert!(self.0 < CHUNK_SIZE as u8 && self.1 < CHUNK_SIZE as u8);
    }
}
