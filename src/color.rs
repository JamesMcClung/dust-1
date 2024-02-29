use bevy::prelude::*;

use crate::schedule::SimSet;
use crate::sim::{Particle, ParticleCoords, PropertyGrid};
use crate::sim::gas::{GasProperties, NORMAL_GAS_DENSITY};

pub struct ColorPlugin;

impl Plugin for ColorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_colors.in_set(SimSet::Recolor));
    }
}


static VACUUM_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);
static AIR_COLOR: Color = Color::rgba(0.0, 0.9, 0.9, 0.2);

fn update_colors(
    particle_grid: Query<&PropertyGrid<Particle>>,
    mut coords: Query<(&ParticleCoords, &mut Sprite)>,
) {
    let particle_grid = particle_grid.single();
    for (coords, mut sprite) in coords.iter_mut() {
        sprite.color = get_color(particle_grid.get(coords.x, coords.y));
    }
}

fn get_color(particle: &Particle) -> Color {
    match particle {
        Particle::Vacuum => VACUUM_COLOR,
        Particle::Air { gas_properties: GasProperties { mass: density, .. } } => AIR_COLOR.with_a(0.01 + 0.99 * density / NORMAL_GAS_DENSITY)
    }
}
