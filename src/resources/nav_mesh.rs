use bevy::prelude::*;
use bevy::tasks::Task;
use std::mem;

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

#[derive(Debug, Default)]
pub struct NavMeshResource(pub NavMesh);

pub type NavMesh = MultiMap<UVec2, Move>;

#[derive(Default)]
pub struct NavMeshCalculationQueue {
    current_task: Option<Task<NavMesh>>,
    next_task: Option<Task<NavMesh>>,
}

impl NavMeshCalculationQueue {
    pub fn peek(&mut self) -> &mut Option<Task<NavMesh>> {
        &mut self.current_task
    }

    pub fn add(&mut self, task: Task<NavMesh>) {
        // TODO properly cancel next_task when reassigning to it
        if self.current_task.is_none() {
            self.current_task = Some(task);
        } else {
            self.next_task = Some(task);
        }
    }

    pub fn pop(&mut self) {
        mem::swap(&mut self.next_task, &mut self.current_task);
        self.next_task = None;
    }
}
