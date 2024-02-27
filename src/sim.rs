use bevy::prelude::*;

const N_PIXELS: ParticleCoords = ParticleCoords::new(128, 128);
const PIXEL_SIZE: Vec2 = Vec2::new(4.0, 4.0);

pub struct SimPlugin;

impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_particles);
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

fn spawn_particles(mut commands: Commands) {
    let corner = Vec2::new(
        -PIXEL_SIZE.x * N_PIXELS.x as f32 / 2.0,
        -PIXEL_SIZE.y * N_PIXELS.y as f32 / 2.0,
    ) + PIXEL_SIZE / 2.0;

    for x in 0..N_PIXELS.x {
        for y in 0..N_PIXELS.y {
            commands.spawn((
                ParticleCoords::new(x, y),
                SpriteBundle {
                    sprite: Sprite {
                        color: random_color(),
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

fn random_color() -> Color {
    Color::rgb(rand::random(), rand::random(), rand::random())
}
