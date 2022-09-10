//! A crate implementing a tilemap for the bevy game engine
//!
//! Includes support for animated tiles

#![warn(missing_docs)]

use std::{marker::PhantomData, mem};

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use rendering::MeshBuilder;
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

/// Stage label for stages related to tilemap rendering
#[derive(Debug, SystemLabel)]
pub enum RenderLabel {
    /// When chunk meshes are generated; before [`Animation`](Self::Animation)
    MeshGeneration,
    /// When mesh animation is applied; after [`MeshGeneration`](Self::MeshGeneration)
    Animation,
}

/// The Bevy plugin to add support for a [`Tilemap`] with a specific tile type
///
/// If using multiple tilemaps, a plugin must be added for each and
/// they must use different structs for tiles
#[derive(Debug)]
pub struct TilemapPlugin<T: Tile> {
    _phantom: PhantomData<Tilemap<T>>,
}

impl<T: Tile> TilemapPlugin<T> {
    /// Creates a new plugin for the given tilemap
    pub fn new() -> Self {
        TilemapPlugin {
            _phantom: PhantomData::default(),
        }
    }
}

impl<T: Tile> Default for TilemapPlugin<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Tile> Plugin for TilemapPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_tilemap_system::<T>)
            .add_system_set(
                SystemSet::new()
                    .label(RenderLabel::MeshGeneration)
                    .with_system(generate_meshes_system::<T>),
            );
    }
}

fn spawn_tilemap_system<T: Tile>(
    mut commands: Commands,
    mut materials: ResMut<Assets<<<T as Tile>::MeshBuilder as MeshBuilder>::Material>>,
) {
    commands.insert_resource(Tilemap::<T>::new(materials.add(T::MeshBuilder::material())))
}

fn generate_meshes_system<T: Tile>(
    mut commands: Commands,
    mut tilemap: ResMut<Tilemap<T>>,
    mut mesh_query: Query<&mut Mesh2dHandle>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let material = tilemap.material().clone();
    for (chunk_pos, chunk) in tilemap
        .iter_chunk_positions_mut()
        .filter(|(_, chunk)| chunk.regenerate_mesh)
    {
        let mut mesh_builder = T::MeshBuilder::init(mem::take(&mut chunk.mesh_carry_data));
        for (tile_pos, tile) in chunk.iter_tile_positions_mut() {
            mesh_builder.set_offset(tile_pos.as_ivec2().as_vec2());
            tile.add_to_mesh(&mut mesh_builder);
        }
        let (new_mesh, carry_data) = mesh_builder.finish();

        let new_mesh = meshes.add(new_mesh).into();
        chunk.mesh_carry_data = carry_data;
        if let Some(mut mesh) = chunk
            .mesh_entity
            .and_then(|entity| mesh_query.get_mut(entity).ok())
        {
            *mesh = new_mesh;
        } else {
            chunk.mesh_entity = Some(
                commands
                    .spawn_bundle(MaterialMesh2dBundle {
                        mesh: new_mesh,
                        transform: Transform::from_translation(Vec3::new(
                            (CHUNK_SIZE as i32 * chunk_pos.x) as f32,
                            (CHUNK_SIZE as i32 * chunk_pos.y) as f32,
                            0.0,
                        )),
                        material: material.clone(),
                        ..default()
                    })
                    .id(),
            );
        }
    }
}
