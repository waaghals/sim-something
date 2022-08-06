use std::cmp::{max, min};
use std::mem;
use std::time::Duration;

use bevy::tasks::AsyncComputeTaskPool;
use bevy::utils::hashbrown::HashMap;
use bevy::utils::Instant;
use bevy::{log, prelude::*};
use futures_lite::future;
use pathfinding::prelude::*;

use crate::components::path_finding::path::*;
use crate::map::world2d_to_grid;
use crate::resources::nav_mesh::{Move, NavMesh};

use super::{DIAGONAL_COST, STRAIGHT_COST};

struct PathFindingRequest {
    from: UVec2,
    to: UVec2,
}

#[derive(Default)]
pub struct PathFindingRequests {
    requests: HashMap<Entity, PathFindingRequest>,
}

#[derive(Default)]
pub struct Navigation {
    pub mesh: NavMesh,
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
    destination_query: Query<(Entity, &Destination, &Transform), Added<Destination>>,
) {
    for (entity, destination, transform) in destination_query.iter() {
        let current_tile = world2d_to_grid(&transform.translation.truncate());

        path_finding_tasks.request(
            entity,
            PathFindingRequest {
                from: current_tile,
                to: destination.0,
            },
        );

        // commands.entity(entity).remove::<Destination>();
    }
}

// TODO in new system, upon a new-map event, remove the Path component from all entities that have it
pub fn calculate_paths(
    mut commands: Commands,
    pool: Res<AsyncComputeTaskPool>,
    navigation: Res<Navigation>,
    mut path_finding_tasks: ResMut<PathFindingRequests>,
) {
    let now = Instant::now();
    let max_duration = Duration::from_millis(1);
    let requests = path_finding_tasks.take();
    for (entity, request) in requests {
        // TODO share this mesh over the treads without clone, as it is quite heavy, it is slowing down scheduling
        let thread_mesh = navigation.mesh.clone();
        let task = pool.spawn(async move {
            astar(
                &request.from,
                |node| neighbours(&thread_mesh, node),
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

fn neighbours(mesh: &NavMesh, current: &UVec2) -> Vec<(UVec2, u32)> {
    match mesh.get_vec(current) {
        None => Vec::new(),
        Some(neighbours) => neighbours
            .iter()
            .map(|neighbour: &Move| (neighbour.destination, neighbour.cost))
            // TODO, do not collect here, but return the iterator itself
            .collect(),
    }
}
