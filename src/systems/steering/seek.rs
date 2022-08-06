use bevy::prelude::*;

use crate::components::steering::{
    behaviour::Seek,
    boid::{Acceleration, Mass, MaxForce, MaxSpeed, Velocity},
};

pub fn seek(
    _time: Res<Time>,
    mut paths_query: Query<(
        Entity,
        &mut Acceleration,
        &Velocity,
        &Transform,
        &Seek,
        &Mass,
        &MaxForce,
        &MaxSpeed,
    )>,
) {
    for (_entity, mut acceleration, velocity, transform, seek, mass, max_force, max_speed) in
        paths_query.iter_mut()
    {
        let max_speed = max_speed.0; // * time.delta_seconds();
        let max_force = max_force.0; // * time.delta_seconds();
        let _mass = mass.0; // * time.delta_seconds();

        let current_position = transform.translation.truncate();
        let mut force = seek.target - current_position;
        force = force.normalize_or_zero() * max_speed;

        let steering = force - velocity.0;
        let steering = steering.clamp_length_max(max_force);

        acceleration.0 += steering;
    }
}
