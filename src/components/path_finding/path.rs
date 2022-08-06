use bevy::{prelude::*, tasks::Task};
use bevy_inspector_egui::Inspectable;

#[derive(Component, Inspectable)]
pub struct Destination(pub UVec2);

#[derive(Component)]
pub struct PendingPath(pub Task<Option<FoundPath>>);

#[derive(Component, Inspectable)]
pub struct FoundPath(pub Vec<UVec2>);
