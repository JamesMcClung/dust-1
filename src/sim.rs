mod particle;
mod particle_grid;

use bevy::prelude::*;

use crate::schedule::SimSet;
use crate::camera::grid_to_camera;

pub use particle::Particle;
pub use particle_grid::PropertyGrid;

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
            .add_systems(PostStartup, (spawn_grids, spawn_particles).chain())
            .add_systems(Update, update_colors.in_set(SimSet::Recolor));
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

fn spawn_grids(mut commands: Commands) {
    commands.spawn(PropertyGrid::<Particle>::default());
}

fn spawn_particles(mut commands: Commands, particle_grid: Query<&PropertyGrid<Particle>>) {
    let particle_grid = particle_grid.single();

    for x in 0..N_PIXELS.x {
        for y in 0..N_PIXELS.y {
            commands.spawn((
                ParticleCoords::new(x, y),
                SpriteBundle {
                    sprite: Sprite {
                        color: particle_grid.get(x, y).get_color(),
                        ..default()
                    },
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

fn update_colors(particle_grid: Query<&PropertyGrid<Particle>>, mut coords: Query<(&ParticleCoords, &mut Sprite)>) {
    let particle_grid = particle_grid.single();
    for (coords, mut sprite) in coords.iter_mut() {
        sprite.color = particle_grid.get(coords.x, coords.y).get_color();
    }
}