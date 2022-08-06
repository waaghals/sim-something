use bevy::prelude::*;
use bevy_ecs_tilemap::map::{
    TilemapGridSize, TilemapId, TilemapSize, TilemapTexture, TilemapTextureSize, TilemapTileSize,
};
use bevy_ecs_tilemap::tiles::{TileBundle, TilePos, TileStorage, TileTexture};
use bevy_ecs_tilemap::{TilemapBundle, TilemapPlugin};
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::components::path_finding::grid::Walkable;
use crate::TILE_SIZE;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_map)
            .add_system(update_tile_storage)
            // .add_system_to_stage(CoreState::PostUpdate, remove_tiles_from_storage)
            .add_plugin(TilemapPlugin);
    }
}

// TODO store previous position in another component to be able to
// synthesize a pos for deletion from the storage?
fn remove_tiles_from_storage(
    removals: RemovedComponents<TilePos>,
    mut map_query: Query<&mut TileStorage>,
) {
    for _map in map_query.iter_mut() {}
    for _entity in removals.iter() {}
}

fn update_tile_storage(
    mut commands: Commands,
    tiles_query: Query<(Entity, &TilePos, &TilemapId), Changed<TilePos>>,
    mut map_query: Query<&mut TileStorage>,
) {
    for (tile_entity, tile_position, tile_map) in tiles_query.iter() {
        if let Ok(mut map) = map_query.get_mut(tile_map.0) {
            map.set(tile_position, Some(tile_entity));
            commands.entity(tile_map.0).add_child(tile_entity);
        }
    }
}

pub fn create_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let floor1 = File::open("assets/floor1.txt").expect("floor1.txt not found");
    let _floor2 = File::open("assets/floor2.txt").expect("floor2.txt not found");
    let texture_handle: Handle<Image> = asset_server.load("tiles_new.png");

    let map_level1 = create_map_entity("Level 1", &mut commands, texture_handle, 0.0);
    // let map_level2 = create_map_entity("Level 2", &mut commands, texture_handle, 1.0);

    create_tile_entities(&mut commands, floor1, map_level1);
    // create_tile_entities(&mut commands, floor2, map_level2);
}

const MAP_SIZE: u8 = 255;
fn create_map_entity(
    name: &str,
    commands: &mut Commands,
    texture_handle: Handle<Image>,
    _z_index: f32,
) -> Entity {
    let tilemap_size = TilemapSize {
        x: MAP_SIZE as u32,
        y: MAP_SIZE as u32,
    };
    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };

    let map = commands
        .spawn()
        .insert(Name::from(name))
        .insert_bundle(TilemapBundle {
            grid_size: TilemapGridSize { x: 16.0, y: 16.0 },
            size: tilemap_size,
            storage: TileStorage::empty(tilemap_size),
            texture_size: TilemapTextureSize { x: 96.0, y: 16.0 },
            texture: TilemapTexture(texture_handle),
            tile_size: tile_size,
            transform: Transform {
                translation: Vec3::ZERO,
                ..Default::default()
            },
            ..Default::default()
        })
        .id();
    map
}

fn create_tile_entities(commands: &mut Commands, file: File, map: Entity) {
    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                let mut entity = commands.spawn();
                entity.insert_bundle(TileBundle {
                    position: TilePos {
                        x: x as u32,
                        y: y as u32,
                    },
                    texture: TileTexture(char_to_texture_index(char)),
                    tilemap_id: TilemapId(map),
                    ..Default::default()
                });

                if char == '.' {
                    entity.insert(Walkable::default());
                }
            }
        }
    }
}

fn char_to_texture_index(char: char) -> u32 {
    match char {
        'W' => 4,
        '.' => 5,
        'U' => 2,
        'D' => 3,
        _ => 0,
    }
}

pub fn world2d_to_grid(transform: &Vec2) -> UVec2 {
    let tile_x = (transform.x / TILE_SIZE).floor() as u32;
    let tile_y = (transform.y / TILE_SIZE).floor() as u32;

    UVec2::new(tile_x, tile_y)
}

pub fn grid_to_world2d(position: &UVec2) -> Vec2 {
    // Grid origin is at bottom left
    let new_x = (position.x as f32 * TILE_SIZE) + (TILE_SIZE / 2.0);
    let new_y = (position.y as f32 * TILE_SIZE) + (TILE_SIZE / 2.0);

    Vec2::new(new_x, new_y)
}
