use bevy::prelude::*;

use crate::sim::{Coords, GRID_CORNER, PIXEL_SIZE};
use crate::sim::types::Vector;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn window_to_camera(window_pos: Vec2, window: &Window, camera: &Transform) -> Vec3 {
    let window_pos = Vec3::new(
        window_pos.x - window.width() / 2.,
        -window_pos.y + window.height() / 2.,
        0.,
    );

    *camera * window_pos
}

pub fn camera_to_grid(camera_pos: Vec3) -> Vector {
    (camera_pos.xy() - GRID_CORNER) / PIXEL_SIZE
}

pub fn grid_to_camera(grid_coords: Coords) -> Vec3 {
    Vec3::new(
        GRID_CORNER.x + grid_coords.x as f32 * PIXEL_SIZE.x,
        GRID_CORNER.y + grid_coords.y as f32 * PIXEL_SIZE.y,
        1.0,
    )
}
