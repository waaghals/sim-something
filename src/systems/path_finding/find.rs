use std::cmp::{max, min};
use std::collections::hash_map::DefaultHasher;
use std::hash::{BuildHasher, BuildHasherDefault, Hasher};
use std::mem;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use bevy::tasks::AsyncComputeTaskPool;
use bevy::utils::hashbrown::HashMap;
use bevy::utils::Instant;
use bevy::{log, prelude::*};
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};
use bevy_inspector_egui::egui::remap;
use futures_lite::future;
use pathfinding::prelude::*;

use crate::components::dna::Dna;
use crate::components::path_finding::grid::Walkable;
use crate::components::path_finding::path::*;
use crate::map::world2d_to_grid;
use crate::resources::nav_mesh::{Move, NavMesh};

use super::{DIAGONAL_COST, STRAIGHT_COST};

struct PathFindingRequest {
    from: UVec2,
    to: UVec2,
    seed: u64,
}

#[derive(Default)]
pub struct PathFindingRequests {
    requests: HashMap<Entity, PathFindingRequest>,
}

#[derive(Default)]
pub struct Navigation {
    pub mesh: Arc<Mutex<NavMesh>>,
}

impl PathFindingRequests {
    fn request(&mut self, entity: Entity, req: PathFindingRequest) {
        self.requests.insert(entity, req);
    }

    fn take(&mut self) -> HashMap<Entity, PathFindingRequest> {
        mem::take(&mut self.requests)
    }
}

pub fn schedule_new_path_finding(
    mut path_finding_tasks: ResMut<PathFindingRequests>,
    destination_query: Query<(Entity, &Destination, &Transform, &Dna), Added<Destination>>,
) {
    for (entity, destination, transform, dna) in destination_query.iter() {
        let current_tile = world2d_to_grid(&transform.translation.truncate());

        path_finding_tasks.request(
            entity,
            PathFindingRequest {
                from: current_tile,
                to: destination.0,
                seed: dna.0,
            },
        );

        // commands.entity(entity).remove::<Destination>();
    }
}

// TODO in new system, upon a new-map event, remove the Path component from all entities that have it
pub fn calculate_paths(
    mut commands: Commands,
    navigation: Res<Arc<Mutex<NavMesh>>>,
    mut path_finding_tasks: ResMut<PathFindingRequests>,
    map_query: Query<&TileStorage>,
    walkable_query: Query<Entity, With<Walkable>>,
) {
    // TODO track the changes of this component and store into resource
    let walkable_tiles = Arc::new(walkable_query.iter().collect::<Vec<Entity>>());

    let map = Arc::new(map_query.single().clone());
    let pool = AsyncComputeTaskPool::get();
    let now = Instant::now();
    let max_duration = Duration::from_millis(1);
    let requests = path_finding_tasks.take();

    for (entity, request) in requests {
        let thread_map = map.clone();
        let thread_mesh = navigation.clone();
        let walkable_tiles = walkable_tiles.clone();
        let task = pool.spawn(async move {
            // TODO share BuildHasherDefault for each call to successors
            astar(
                &request.from,
                |node| {
                    add_entity_tie_breaker(
                        request.seed,
                        BuildHasherDefault::<DefaultHasher>::default(),
                        neighbours(&thread_map, &walkable_tiles, thread_mesh.clone(), *node),
                    )
                },
                |node| heuristic(node, &request.to),
                |node| node.x == request.to.x && node.y == request.to.y,
            )
            .map(|path| FoundPath(path.0))
        });

        commands.entity(entity).insert(PendingPath(task));

        if now.elapsed() > max_duration {
            break;
        }
    }
}

pub fn handle_completed_path(
    mut commands: Commands,
    mut transform_tasks: Query<(Entity, &mut PendingPath)>,
) {
    for (entity_id, mut pending_path) in transform_tasks.iter_mut() {
        if let Some(completion) = future::block_on(future::poll_once(&mut pending_path.0)) {
            let mut entity = commands.entity(entity_id);
            entity.remove::<PendingPath>();

            if let Some(path) = completion {
                entity.insert(path);
            } else {
                log::info!("Could not find path for entity {:#?}", entity_id);
            }
        }
    }
}

fn heuristic(from: &UVec2, to: &UVec2) -> u32 {
    let dx = from.x.abs_diff(to.x);
    let dy = from.y.abs_diff(to.y);

    STRAIGHT_COST * max(dx, dy) + (DIAGONAL_COST - STRAIGHT_COST) * min(dx, dy)
}

fn add_entity_tie_breaker<H, I>(
    seed: u64,
    hasher_builder: H,
    moves: I,
) -> impl Iterator<Item = (UVec2, u32)>
where
    H: BuildHasher,
    I: IntoIterator<Item = (UVec2, u32)>,
{
    moves.into_iter().map(move |(position, weight)| {
        let mut hasher = hasher_builder.build_hasher();
        hasher.write_u32(position.x);
        hasher.write_u32(position.y);
        hasher.write_u64(seed);
        let hash: f64 = (hasher.finish() as u32).into();
        // TODO implement own remap
        let tie_breaker = remap(hash, 0.0f64..=f64::MAX, 0.0f64..=10.0f64) as u32;

        (position, weight + tie_breaker)
    })
}

fn neighbours(
    storage: &TileStorage,
    walkable: &[Entity],
    mesh: Arc<Mutex<NavMesh>>,
    current: UVec2,
) -> Vec<(UVec2, u32)> {
    // TODO cleanup, extract methods and after new neighbours calculation, simply call the nav_mesh again instead of remembering them during calculation
    let mut nav_mesh = mesh.lock().unwrap();
    match nav_mesh.get_vec(&current) {
        None => {
            let tile_pos = TilePos::from(current);
            let mut neighbours = Vec::with_capacity(8usize);

            match storage.get(&tile_pos) {
                Some(current_entity) => {
                    if walkable.contains(&current_entity) {
                        let [north, south, west, east, northwest, northeast, southwest, southeast] =
                            storage.get_neighboring_pos(&tile_pos);
                        for neighbour in [north, south, west, east].into_iter().flatten() {
                            if let Some(neighbour_entity) = storage.get(&neighbour) {
                                if walkable.contains(&neighbour_entity) {
                                    nav_mesh.insert(
                                        current,
                                        Move {
                                            destination: neighbour.into(),
                                            cost: STRAIGHT_COST,
                                        },
                                    );
                                    neighbours.push(Move {
                                        destination: neighbour.into(),
                                        cost: STRAIGHT_COST,
                                    });
                                }
                            }
                        }
                        for neighbour in [northwest, northeast, southwest, southeast]
                            .into_iter()
                            .flatten()
                        {
                            if let Some(neighbour_entity) = storage.get(&neighbour) {
                                if walkable.contains(&neighbour_entity) {
                                    nav_mesh.insert(
                                        current,
                                        Move {
                                            destination: neighbour.into(),
                                            cost: DIAGONAL_COST,
                                        },
                                    );
                                    neighbours.push(Move {
                                        destination: neighbour.into(),
                                        cost: DIAGONAL_COST,
                                    });
                                }
                            }
                        }
                    }
                }
                None => todo!(), // There is no entity here, so it is definitally not walkable
            }

            // TODO cleanup
            return neighbours
                .iter()
                .map(|neighbour: &Move| (neighbour.destination, neighbour.cost))
                // TODO, do not collect here, but return the iterator itself
                .collect();
        }
        Some(neighbours) => neighbours
            .iter()
            .map(|neighbour: &Move| (neighbour.destination, neighbour.cost))
            // TODO, do not collect here, but return the iterator itself
            .collect(),
    }
}
