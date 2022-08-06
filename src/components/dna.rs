use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Inspectable)]
pub struct Dna {
    dna: u64,
}

impl Dna {
    pub fn random() -> Dna {
        Dna {
            dna: rand::random(),
        }
    }

    //  fn map(&self, min: u64, max: u64) -> u64 {
    //      min + (self.dna - u64::MIN) * (max - min) / u64::MAX
    //  }

    pub fn max_speed(&self) -> f32 {
        32.0
        //   self.map(3, 10) as f32
    }

    pub fn max_force(&self) -> f32 {
        0.5
        // 0.01
    }

    pub fn mass(&self) -> f32 {
        2.0
    }
}
