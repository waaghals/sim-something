use bevy::prelude::Color;
use bevy_prototype_lyon::prelude::DrawMode;

pub mod path_finding;

pub fn color(draw_mode: &DrawMode) -> Color {
    match draw_mode {
        DrawMode::Fill(mode) => mode.color,
        DrawMode::Stroke(mode) => mode.color,
        DrawMode::Outlined { fill_mode, .. } => fill_mode.color,
    }
}
