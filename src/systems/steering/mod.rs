use bevy::prelude::*;

use bevy_prototype_debug_lines::DebugLines;
use bevy_prototype_lyon::prelude::DrawMode;

use crate::components::steering::boid::{Acceleration, MaxSpeed, Velocity};

use super::debug::color;

pub mod follow_mouse;
pub mod follow_path;
pub mod seek;

pub fn apply(
    mut query: Query<(
        Entity,
        &mut Velocity,
        &mut Transform,
        &mut Acceleration,
        &MaxSpeed,
    )>,
) {
    for (_entity, mut velocity, mut transform, mut acceleration, max_speed) in query.iter_mut() {
        velocity.0 += acceleration.0;
        velocity.0 = velocity.0.clamp_length_max(max_speed.0);

        let z = transform.translation.z;
        let new_translation = transform.translation.truncate() + velocity.0;
        transform.translation = new_translation.extend(z);

        acceleration.0 = Vec2::ZERO;
    }
}

pub fn debug(query: Query<(&Velocity, &Transform, &DrawMode)>, mut lines: ResMut<DebugLines>) {
    for (velocity, transform, draw_mode) in query.iter() {
        let color = color(draw_mode);

        let current_position = transform.translation.truncate();
        let destination = current_position + velocity.0;

        lines.line_colored(
            current_position.extend(0.0),
            destination.extend(0.0),
            0.0,
            color,
        );
    }
}
