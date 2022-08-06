use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Inspectable)]
pub struct Dna(pub u64);

impl Dna {
    pub fn random() -> Dna {
        Dna(rand::random())
    }
}
