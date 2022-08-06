use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use core::panic;

#[derive(Component, Inspectable)]
pub struct Seek {
    pub target: Vec2,
}

#[derive(Component, Inspectable)]
pub struct Flee(pub Vec2);

#[derive(Component, Inspectable)]
pub struct Pursuit {
    target_current_position: Vec2,
    target_current_velocity: Vec2,
    lookahead: f32,
}

#[derive(Component, Inspectable)]
pub struct Evade;

#[derive(Component, Inspectable)]
pub struct Arive {
    pub target: Vec2,
    pub radius: Vec2,
}

#[derive(Component, Inspectable)]
pub struct Avoid;

#[derive(Component, Inspectable)]
pub struct Wander;

#[derive(Component, Inspectable)]
#[non_exhaustive]
pub struct FollowPath {
    pub path: Vec<Vec2>,
    pub path_width: f32,
    pub lookahead: f32,
}

impl FollowPath {
    pub fn new(path: Vec<Vec2>, path_width: f32, lookahead: f32) -> FollowPath {
        if path.len() < 2 {
            panic!("Path must contain at least one segment");
        }

        FollowPath {
            path,
            path_width,
            lookahead,
        }
    }
}

#[derive(Component, Inspectable)]
pub struct Separation;

#[derive(Component, Inspectable)]
pub struct Cohesion;

#[derive(Component, Inspectable)]
pub struct Alignment;

#[derive(Component, Inspectable)]
pub struct FollowLeader;

#[derive(Component, Inspectable)]
pub struct Interpose;
