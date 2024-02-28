use bevy::prelude::*;

use crate::camera::{camera_to_grid, window_to_camera};
use crate::sim::{Particle, PropertyGrid};
use crate::schedule::SimSet;

pub struct DrawPlugin;

impl Plugin for DrawPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_particle.in_set(SimSet::Recolor));
    }
}

fn draw_particle(
    mut particle_grid: Query<&mut PropertyGrid<Particle>>,
    cursor_input: Res<ButtonInput<MouseButton>>,
    window: Query<&Window>,
    camera: Query<&Transform, With<Camera>>,
) {
    let window = window.single();
    let camera = camera.single();

    if cursor_input.pressed(MouseButton::Left) {
        if let Some(cursor_position) = window.cursor_position() {
            if let Some(grid_coords) = camera_to_grid(window_to_camera(cursor_position, window, camera)) {
                *particle_grid.single_mut().get_mut(grid_coords.x, grid_coords.y) = Particle::Air;
            }
        }
    }
}
