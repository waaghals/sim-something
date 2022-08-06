use bevy::prelude::*;

use bevy_inspector_egui::Inspectable;

use multimap::MultiMap;

#[derive(Inspectable, Clone, Debug)]
pub struct Move {
    pub destination: UVec2,
    pub cost: u32,
}

impl Default for Move {
    fn default() -> Self {
        Move {
            destination: UVec2::ZERO,
            cost: 0,
        }
    }
}

pub type NavMesh = MultiMap<UVec2, Move>;
