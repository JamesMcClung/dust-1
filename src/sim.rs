mod particle;
mod particle_grid;

use bevy::prelude::*;

use particle::Particle;
use particle_grid::PropertyGrid;

const N_PIXELS: ParticleCoords = ParticleCoords::new(128, 128);
const PIXEL_SIZE: Vec2 = Vec2::new(4.0, 4.0);

pub struct SimPlugin;

impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, (spawn_grids, spawn_particles).chain());
    }
}

#[derive(Component)]
struct ParticleCoords {
    x: usize,
    y: usize,
}

impl ParticleCoords {
    const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

fn spawn_grids(mut commands: Commands) {
    commands.spawn(PropertyGrid::<Particle>::default());
}

fn spawn_particles(mut commands: Commands, particle_grid: Query<&PropertyGrid<Particle>>) {
    let corner = Vec2::new(
        -PIXEL_SIZE.x * N_PIXELS.x as f32 / 2.0,
        -PIXEL_SIZE.y * N_PIXELS.y as f32 / 2.0,
    ) + PIXEL_SIZE / 2.0;

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
                        translation: Vec3::new(
                            corner.x + x as f32 * PIXEL_SIZE.x,
                            corner.y + y as f32 * PIXEL_SIZE.y,
                            1.0,
                        ),
                        scale: PIXEL_SIZE.extend(0.0),
                        ..default()
                    },
                    ..default()
                },
            ));
        }
    }
}
