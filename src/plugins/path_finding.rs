use std::sync::{Arc, Mutex};

use bevy::prelude::*;

use crate::{
    resources::nav_mesh::NavMesh,
    systems::path_finding::{
        find::{
            calculate_paths, handle_completed_path, schedule_new_path_finding, PathFindingRequests,
        },
        // mesh::calculate_new_nav_mesh,
        steering::transform_path,
    },
};

pub struct PathFindingPlugin;

impl Plugin for PathFindingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Arc::<Mutex<NavMesh>>::default())
            .insert_resource(PathFindingRequests::default())
            .add_system(schedule_new_path_finding)
            .add_system(calculate_paths.after(schedule_new_path_finding))
            .add_system(handle_completed_path)
            // .add_system(calculate_new_nav_mesh)
            .add_system(transform_path);
    }
}
