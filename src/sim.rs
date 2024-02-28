pub mod gas;
mod particle;
mod property_grid;
mod types;


use bevy::prelude::*;

use crate::camera::grid_to_camera;

pub use particle::Particle;
pub use property_grid::PropertyGrid;

use gas::GasPlugin;


pub const N_PIXELS: ParticleCoords = ParticleCoords::new(128, 128);
pub const PIXEL_SIZE: Vec2 = Vec2::new(4.0, 4.0);
pub const GRID_CORNER: Vec2 = Vec2::new(
    PIXEL_SIZE.x / 2.0 - PIXEL_SIZE.x * N_PIXELS.x as f32 / 2.0,
    PIXEL_SIZE.y / 2.0 - PIXEL_SIZE.y * N_PIXELS.y as f32 / 2.0,
);

pub struct SimPlugin;

impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostStartup, (spawn_particle_grid, spawn_sprites).chain())
            .add_plugins(GasPlugin);
    }
}

#[derive(Component)]
pub struct ParticleCoords {
    pub x: usize,
    pub y: usize,
}

impl ParticleCoords {
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

fn spawn_particle_grid(mut commands: Commands) {
    commands.spawn(PropertyGrid::<Particle>::default());
}

fn spawn_sprites(mut commands: Commands) {
    for x in 0..N_PIXELS.x {
        for y in 0..N_PIXELS.y {
            commands.spawn((
                ParticleCoords::new(x, y),
                SpriteBundle {
                    transform: Transform {
                        translation: grid_to_camera(ParticleCoords::new(x, y)),
                        scale: PIXEL_SIZE.extend(0.0),
                        ..default()
                    },
                    ..default()
                },
            ));
        }
    }
}
