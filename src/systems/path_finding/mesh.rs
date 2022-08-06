use bevy::{log, prelude::*};
use bevy_ecs_tilemap::{
    map::Tilemap2dSize,
    tiles::{Tile2dStorage, TilePos2d},
};
use futures_lite::future;
use tracing::{span, Level};

use crate::{
    components::path_finding::grid::Walkable,
    resources::nav_mesh::{Move, NavMesh, NavMeshCalculationQueue},
    systems::path_finding::{DIAGONAL_COST, STRAIGHT_COST},
};

use super::find::Navigation;

pub fn calculate_new_nav_mesh(
    mut navigation: ResMut<Navigation>,
    keyboard_input: Res<Input<KeyCode>>,
    map_query: Query<(&Tile2dStorage, &Tilemap2dSize)>,
    walkable_query: Query<&Walkable>,
) {
    if !keyboard_input.just_pressed(KeyCode::M) {
        return;
    }

    let span = span!(Level::INFO, "calculate_new_nav_mesh");
    let _enter = span.enter();

    let (storage, size) = map_query.single();

    // TODO figure our how to get this into a thread
    let mut mesh = NavMesh::default();
    let mut x = 0;
    let mut y = 0;
    while x < size.x {
        while y < size.y {
            log::error!("Checking position x: {} y: {}", x, y);
            let pos = TilePos2d { x, y };
            match storage.get(&pos) {
                None => {
                    log::error!("Storage does not contain position {:#?}", &pos);
                }
                Some(entity) => {
                    if walkable_query.get(entity).is_ok() {
                        let [north, south, west, east, northwest, northeast, southwest, southeast] =
                            storage.get_neighboring_pos(&pos);
                        log::debug!("Position {:#?} neighbours {:#?},{:#?},{:#?},{:#?},{:#?},{:#?},{:#?},{:#?} ", &pos, &north, &south, &west, &east, &northwest, &northeast, &southwest, &southeast);
                        for straight_neighbour in [north, south, west, east] {
                            if let Some(neighbour) = straight_neighbour {
                                if let Some(neighbour_entity) = storage.get(&neighbour) {
                                    if walkable_query.contains(neighbour_entity) {
                                        mesh.insert(
                                            pos.into(),
                                            Move {
                                                destination: neighbour.into(),
                                                cost: STRAIGHT_COST,
                                            },
                                        );

                                        log::debug!("Position {:#?} IS walkable!", &pos);
                                    } else {
                                        log::info!("Position {:#?} is not walkable", &pos);
                                    }
                                }
                            }
                        }
                        for diagonal_neighbour in [northwest, northeast, southwest, southeast] {
                            if let Some(neighbour) = diagonal_neighbour {
                                if let Some(neighbour_entity) = storage.get(&neighbour) {
                                    if walkable_query.contains(neighbour_entity) {
                                        mesh.insert(
                                            pos.into(),
                                            Move {
                                                destination: neighbour.into(),
                                                cost: DIAGONAL_COST,
                                            },
                                        );
                                    }
                                    log::debug!("Position {:#?} IS walkable!", &pos);
                                } else {
                                    log::info!("Position {:#?} is not walkable", &pos);
                                }
                            }
                        }
                    } else {
                        log::debug!("Position {:#?} does not contain an entity", &pos);
                    }
                }
            }
            y += 1;
        }
        y = 0;
        x += 1;
    }
    navigation.mesh = mesh;
}

pub fn handle_new_nav_mesh(
    mut navigation: ResMut<Navigation>,
    mut pending_queue: ResMut<NavMeshCalculationQueue>,
) {
    if let Some(task) = pending_queue.peek() {
        match future::block_on(future::poll_once(task)) {
            None => {}
            Some(calculated_nav_mesh) => {
                navigation.mesh = calculated_nav_mesh;
                pending_queue.pop();
            }
        }
    }

    // TODO trigger new-map event
}
