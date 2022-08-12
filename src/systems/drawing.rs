use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage, TileTexture};
use bevy_mouse_tracking_plugin::MousePosWorld;

use crate::{components::path_finding::grid::Walkable, map::world2d_to_grid};

pub fn drawing(
    mut commands: Commands,
    mouse: Res<Input<MouseButton>>,
    mouse_pos: Res<MousePosWorld>,
    map_query: Query<&TileStorage>,
    walkable_query: Query<&Walkable>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let map = map_query.single();
    let mouse_pos = mouse_pos.truncate();
    let tile_pos = world2d_to_grid(&mouse_pos);
    let entity = map.get(&TilePos::from(tile_pos));

    match entity {
        Some(entity) => match walkable_query.get(entity) {
            Ok(_) => {
                commands
                    .entity(entity)
                    .insert(TileTexture(5))
                    .remove::<Walkable>();
            }
            Err(_) => {
                commands
                    .entity(entity)
                    .insert(Walkable)
                    .insert(TileTexture(4));
            }
        },
        None => {}
    }
}
