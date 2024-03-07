mod palette;

use bevy::prelude::*;
use rand::Rng;

use crate::camera::{camera_to_grid, window_to_camera};
use crate::sim::types::Vector;
use crate::sim::{path, Particle, PropertyGrid};
use crate::schedule::SimSet;
use palette::ParticleToDraw;
use rand::rngs::ThreadRng;

#[derive(Component)]
struct LastCursorCoords(Option<Vector>);

pub struct DrawPlugin;

impl Plugin for DrawPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(palette::PalettePlugin)
            .add_systems(Startup, add_last_cursor_coords)
            .add_systems(Update, draw_particle.in_set(SimSet::Recolor));
    }
}

fn add_last_cursor_coords(mut commands: Commands) {
    commands.spawn(LastCursorCoords(None));
}

fn draw_particle(
    particle_to_draw: Query<&ParticleToDraw>,
    mut particle_grid: Query<&mut PropertyGrid<Particle>>,
    mut last_cursor_coords: Query<&mut LastCursorCoords>,
    cursor_input: Res<ButtonInput<MouseButton>>,
    window: Query<&Window>,
    camera: Query<&Transform, With<Camera>>,
) {
    let ParticleToDraw(Some(particle_to_draw)) = particle_to_draw.single() else {
        return;
    };
    let mut particle_grid = particle_grid.single_mut();
    let mut last_cursor_coords = last_cursor_coords.single_mut();

    let window = window.single();
    let camera = camera.single();

    if cursor_input.pressed(MouseButton::Left) {
        if let Some(cursor_position) = window.cursor_position() {
            let end = camera_to_grid(window_to_camera(cursor_position, window, camera));
            let start = last_cursor_coords.0.unwrap_or(end);

            let mut rng = rand::thread_rng();

            for coords in path::get_path(start, end) {
                if let Some(particle) = particle_grid.try_get_mut(coords) {
                    *particle = randomize_internal_position(&mut rng, particle_to_draw.clone());
                }
            }

            last_cursor_coords.0 = Some(end);
        }
    } else {
        last_cursor_coords.0 = None;
    }
}

fn randomize_internal_position(rng: &mut ThreadRng, mut particle: Particle) -> Particle {
    if let Some(internal_position) = particle.get_physical_properties_mut() {
        internal_position.internal_position.x = rng.gen();
        internal_position.internal_position.y = rng.gen();
        particle
    } else {
        particle
    }
}
