use bevy::math::vec3;
use bevy::prelude::*;

use bevy_mouse_tracking_plugin::MousePosWorld;
use bevy_prototype_lyon::prelude::*;

use crate::components::dna::Dna;
use crate::components::path_finding::path::Destination;
use crate::components::steering::boid::{Acceleration, Mass, MaxForce, MaxSpeed, Velocity};
use rand::Rng;

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_actor);
    }
}

fn spawn_actor(
    mut commands: Commands,
    mouse: Res<Input<MouseButton>>,
    mouse_pos: Res<MousePosWorld>,
) {
    if !mouse.pressed(MouseButton::Right) {
        return;
    }

    let mut rng = rand::thread_rng();

    let circle = shapes::Circle {
        radius: 6.0,
        center: Vec2::ZERO,
    };

    let x = rng.gen_range(0..255) as u32;
    let y = rng.gen_range(0..255) as u32;
    let destination_tile = UVec2::new(x, y);

    let color = Color::from([rng.gen(), rng.gen(), rng.gen()]);

    commands
        .spawn()
        .insert(Name::new("Actor"))
        .insert(Dna::random())
        .insert(Velocity(Vec2::ONE))
        .insert(Mass(2.0))
        .insert(MaxSpeed(1.0))
        .insert(MaxForce(0.5))
        .insert(Acceleration(Vec2::ZERO))
        .insert_bundle(GeometryBuilder::build_as(
            &circle,
            DrawMode::Outlined {
                fill_mode: FillMode::color(color),
                outline_mode: StrokeMode::new(Color::BLACK, 1.0),
            },
            Transform {
                translation: vec3(mouse_pos.x, mouse_pos.y, 900.0),
                ..Default::default()
            },
        ))
        .insert(Destination(destination_tile));
}
