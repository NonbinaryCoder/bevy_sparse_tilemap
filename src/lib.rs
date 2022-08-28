//! A crate implementing a tilemap for the bevy game engine
//!
//! Includes support for animated tiles

#![warn(missing_docs)]

use std::marker::PhantomData;

use bevy::prelude::Plugin;
use tile::Tile;
use tilemap::Tilemap;

/// The width/height of tilemap chunks
///
/// Value may change in the future, but MUST be a power of 2
pub const CHUNK_SIZE: usize = 32;

pub mod animation;
pub mod rendering;
pub mod tile;
pub mod tilemap;

/// The Bevy plugin to add support for a [`Tilemap`] with a specific tile type
///
/// If using multiple tilemaps, a plugin must be added for each and
/// they must use different structs for tiles
#[derive(Debug, Default)]
pub struct TilemapPlugin<T: Tile>(PhantomData<Tilemap<T>>);

impl<T: Tile> Plugin for TilemapPlugin<T> {
    fn build(&self, _app: &mut bevy::prelude::App) {
        todo!()
    }
}
