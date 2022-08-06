use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

use crate::components::steering::{
    behaviour::{FollowPath, Seek},
    boid::Velocity,
};

/// Remove already visisted segments from the path
pub fn path_culling(_commands: Commands, _paths_query: Query<(&Transform, &FollowPath)>) {}

pub fn follow_path(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Velocity, &FollowPath)>,
    mut lines: ResMut<DebugLines>,
) {
    for (entity, transform, velocity, follow_path) in query.iter() {
        let current_position = transform.translation.truncate();
        let lookahead_velocity = velocity.0.normalize_or_zero() * follow_path.lookahead;
        let estimated_future_location = current_position + lookahead_velocity;

        let closest_point = closest_point(&follow_path.path, estimated_future_location);

        lines.line_colored(
            current_position.extend(0.0),
            closest_point.extend(0.0),
            0.0,
            Color::RED,
        );

        if on_path(
            follow_path.path_width,
            closest_point,
            estimated_future_location,
        ) {
            // already on the path, no need to correct the velocity.
            // So remove the seek component, so the velocity is not changed
            commands.entity(entity).remove::<Seek>();
            continue;
        }

        // TODO target a point slightly ahead of the current closest point.
        // But that will need to find the closests line segment first,
        // which needs to be refactored out from the closest_point function
        commands.entity(entity).insert(Seek {
            target: closest_point,
        });
    }
}

fn closest_point_on_line(a: Vec2, b: Vec2, from: Vec2) -> Vec2 {
    // Perform scaler projection to find closest point on the path line
    let ap = from - a;
    let ab = b - a;

    let mut ab = ab.normalize();
    ab *= ap.dot(ab);

    a + ab
}

fn closest_point(path: &Vec<Vec2>, from: Vec2) -> Vec2 {
    // Find the closests point for each line segments
    // Returns when a local low was found, starting from the first segment
    let mut closest_distance = f32::MAX;
    let mut last_distance = f32::MAX;
    let mut closest_point = Vec2::ZERO;

    // The path always has at least two vectors (= one segment)
    let mut previous = path.get(0).unwrap();
    for current in path.iter().skip(1) {
        let mut closest = closest_point_on_line(*previous, *current, from);
        // The normal point might be off the segment we are checking. (as if the magnitude of the line was infinite)
        // In that case, set the closest point to the end of the current segment we are checking
        if previous.distance(*current) < previous.distance(closest) {
            closest = *current;
        }

        let distance = closest.distance(from);
        if distance < closest_distance {
            closest_distance = distance;
            closest_point = closest;
        }

        if last_distance < distance {
            // We found local minimal distance
            // Return early, to prevent checking all segments
            return closest_point;
        }
        last_distance = distance;
        previous = current;
    }

    closest_point
}

fn on_path(path_width: f32, closest: Vec2, from: Vec2) -> bool {
    let distance = closest.distance(from);
    distance <= path_width
}
