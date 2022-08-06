use bevy::prelude::*;

use crate::systems::steering::{
    apply, follow_path::follow_path, follow_path::path_culling, seek::seek,
};

pub struct SteeringPlugin;

impl Plugin for SteeringPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_system(follow_mouse.before(apply))
            .add_system(follow_path.before(apply))
            .add_system(path_culling.before(apply))
            .add_system(seek.before(apply))
            .add_system(apply);
    }
}
