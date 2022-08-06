use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Inspectable)]
pub struct Mass(pub f32);

#[derive(Component, Inspectable)]
pub struct Velocity(pub Vec2);

#[derive(Component, Inspectable)]
pub struct Acceleration(pub Vec2);

#[derive(Component, Inspectable)]
pub struct MaxForce(pub f32);

#[derive(Component, Inspectable)]
pub struct MaxSpeed(pub f32);
