use crate::components::dna::Dna;
use crate::components::path_finding::path::{Destination, FoundPath};
use crate::components::steering::behaviour::{
    Alignment, Arive, Avoid, Cohesion, Evade, Flee, FollowLeader, FollowPath, Interpose, Pursuit,
    Seek, Separation, Wander,
};
use crate::components::steering::boid::{Acceleration, Mass, MaxForce, MaxSpeed, Velocity};
use crate::systems::debug::color;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::log::{Level, LogSettings};
use bevy::prelude::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};
use bevy_prototype_lyon::prelude::*;
use rand::Rng;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(LogDiagnosticsPlugin::default())
                .add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(WorldInspectorPlugin::new())
                .add_plugin(ShapePlugin)
                .register_inspectable::<FoundPath>()
                .register_inspectable::<Destination>()
                .register_inspectable::<Velocity>()
                .register_inspectable::<Acceleration>()
                .register_inspectable::<Mass>()
                .register_inspectable::<MaxForce>()
                .register_inspectable::<MaxSpeed>()
                .register_inspectable::<Seek>()
                .register_inspectable::<Flee>()
                .register_inspectable::<Pursuit>()
                .register_inspectable::<Evade>()
                .register_inspectable::<Arive>()
                .register_inspectable::<Avoid>()
                .register_inspectable::<Wander>()
                .register_inspectable::<FollowPath>()
                .register_inspectable::<Separation>()
                .register_inspectable::<Cohesion>()
                .register_inspectable::<Alignment>()
                .register_inspectable::<FollowLeader>()
                .register_inspectable::<Interpose>()
                .register_inspectable::<Dna>()
                // .add_startup_system(draw_origin)
                .insert_resource(LogSettings {
                    level: Level::DEBUG,
                    filter: "wgpu=error,bevy_render=info,bevy_ecs=trace".to_string(),
                })
                .add_plugin(DebugLinesPlugin::default())
                .add_system(render_paths)
                .add_system(set_new_destinations);
        }
    }
}

fn draw_origin(mut commands: Commands) {
    let shape = shapes::Circle {
        radius: 6.0,
        center: Vec2::ZERO,
    };

    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::CYAN),
            outline_mode: StrokeMode::new(Color::BLACK, 1.0),
        },
        Transform::default(),
    ));
}

fn render_paths(path_query: Query<(&FollowPath, &DrawMode)>, mut lines: ResMut<DebugLines>) {
    for (path, draw_mode) in path_query.iter() {
        let mut color = color(draw_mode);
        color.set_a(0.5);

        if path.path.is_empty() {
            return;
        }

        let mut prev = path.path.get(0).unwrap();
        for current in &path.path {
            let start = prev.extend(0.0);
            let end = current.extend(0.0);
            lines.line_colored(start, end, 0.0, color);

            prev = current;
        }
    }
}

fn set_new_destinations(
    keyboard: Res<Input<KeyCode>>,
    mut commands: Commands,
    entities: Query<Entity, With<FoundPath>>,
) {
    if !keyboard.just_pressed(KeyCode::R) {
        return;
    }
    let mut rng = rand::thread_rng();

    for entity in entities.iter() {
        let x = rng.gen_range(0..255) as u32;
        let y = rng.gen_range(0..255) as u32;
        let destination_tile = UVec2::new(x, y);

        let mut entity = commands.entity(entity);
        entity.remove::<Destination>();
        entity.remove::<FoundPath>();
        entity.insert(Destination(destination_tile));
    }
}
