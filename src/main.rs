#![allow(clippy::redundant_field_names)]

use bevy::prelude::*;
use bevy_mouse_tracking_plugin::MousePosPlugin;

use debug::DebugPlugin;
use systems::map::set_texture_filters_to_nearest;

use crate::actor::ActorPlugin;
use camera::CameraPlugin;
use map::TileMapPlugin;
use plugins::{path_finding::PathFindingPlugin, steering::SteeringPlugin};

mod actor;
mod camera;
mod components;
mod debug;
mod map;
mod plugins;
mod resources;
mod systems;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 16.0;
pub const TILE_SIZE_DIAGONAL: f32 = 22.6;

fn main() {
    let height = 400.0;
    App::new()
        // .insert_resource(LogSettings {
        //     level: Level::DEBUG,
        //     filter: "wgpu=error,bevy_render=info,bevy_ecs=trace".to_string(),
        // })
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height: height,
            title: "Sim Something".to_string(),
            resizable: true,
            ..Default::default()
        })
        // .add_startup_system(spawn_camera)
        .add_plugins(DefaultPlugins)
        // .add_plugin(AsciiPlugin)
        .add_plugin(ActorPlugin)
        .add_plugin(PathFindingPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(TileMapPlugin)
        .add_system(set_texture_filters_to_nearest)
        .add_plugin(DebugPlugin)
        .add_plugin(SteeringPlugin)
        .add_plugin(MousePosPlugin::SingleCamera)
        .run();
}
