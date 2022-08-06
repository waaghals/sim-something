use bevy::prelude::*;

use crate::{
    components::{path_finding::path::FoundPath, steering::behaviour::FollowPath},
    map::grid_to_world2d,
};

pub fn transform_path(mut commands: Commands, query: Query<(Entity, &FoundPath)>) {
    for (entity, found_path) in query.iter() {
        let path = found_path
            .0
            .iter()
            .map(grid_to_world2d)
            .collect::<Vec<Vec2>>();

        commands
            .entity(entity)
            .remove::<FoundPath>()
            .insert(FollowPath::new(path, 0.0, 10.0));
    }
}
