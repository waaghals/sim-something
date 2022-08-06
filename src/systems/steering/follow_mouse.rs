use bevy::prelude::*;
use bevy_mouse_tracking_plugin::MousePosWorld;

use crate::components::steering::{
    behaviour::Seek,
    boid::{Acceleration, Velocity},
};

pub fn follow_mouse(
    mut commands: Commands,
    query: Query<Entity, (With<Transform>, With<Velocity>, With<Acceleration>)>,
    mouse_pos: Res<MousePosWorld>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(Seek {
            target: mouse_pos.truncate(),
        });
    }
}
