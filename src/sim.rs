pub mod gas;
mod gravity;
mod particle;
mod path;
mod property_grid;
mod types;
mod coords;


use bevy::prelude::*;

use crate::camera::grid_to_camera;

pub use particle::Particle;
pub use property_grid::PropertyGrid;
pub use coords::{Coords, RelCoords};


pub const N_PIXELS: Coords = Coords::new(128, 128);
pub const PIXEL_SIZE: Vec2 = Vec2::new(4.0, 4.0);
pub const GRID_CORNER: Vec2 = Vec2::new(
    PIXEL_SIZE.x / 2.0 - PIXEL_SIZE.x * N_PIXELS.x as f32 / 2.0,
    PIXEL_SIZE.y / 2.0 - PIXEL_SIZE.y * N_PIXELS.y as f32 / 2.0,
);

pub struct SimPlugin;

impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (spawn_particle_grid, spawn_sprites).chain())
            .add_plugins(gravity::GravityPlugin)
            .add_plugins(gas::GasPlugin);
    }
}

fn spawn_particle_grid(mut commands: Commands) {
    commands.spawn(PropertyGrid::<Particle>::default());
}

fn spawn_sprites(mut commands: Commands) {
    for coords in Coords::ZERO.to(N_PIXELS) {
        commands.spawn((
            coords,
            SpriteBundle {
                transform: Transform {
                    translation: grid_to_camera(coords),
                    scale: PIXEL_SIZE.extend(0.0),
                    ..default()
                },
                ..default()
            },
        ));
    }
}
