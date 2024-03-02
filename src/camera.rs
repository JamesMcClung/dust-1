use bevy::prelude::*;

use crate::sim::{GRID_CORNER, N_PIXELS, PIXEL_SIZE, Coords};

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

pub fn camera_to_grid(camera_pos: Vec3) -> Option<Coords> {
    let grid_x = (camera_pos.x - GRID_CORNER.x) / PIXEL_SIZE.x;
    let grid_y = (camera_pos.y - GRID_CORNER.y) / PIXEL_SIZE.y;

    if grid_x < 0.0 || grid_x + 0.5 >= N_PIXELS.x as f32
        || grid_y < 0.0 || grid_y + 0.5 >= N_PIXELS.y as f32
    {
        None
    } else {
        Some(Coords::new((grid_x + 0.5) as usize, (grid_y + 0.5) as usize))
    }
}

pub fn grid_to_camera(grid_coords: Coords) -> Vec3 {
    Vec3::new(
        GRID_CORNER.x + grid_coords.x as f32 * PIXEL_SIZE.x,
        GRID_CORNER.y + grid_coords.y as f32 * PIXEL_SIZE.y,
        1.0,
    )
}
