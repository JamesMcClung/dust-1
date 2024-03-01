use bevy::prelude::*;

use crate::schedule::SimSet;
use crate::sim::gravity::GRAVITY_ACCELERATION;
use crate::sim::types::Scalar;
use crate::sim::{Coords, Particle, PropertyGrid, N_PIXELS};
use crate::sim::gas::GasProperties;

pub struct ColorPlugin;

impl Plugin for ColorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_colors.in_set(SimSet::Recolor));
    }
}


static VACUUM_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);

fn update_colors(
    particle_grid: Query<&PropertyGrid<Particle>>,
    mut coords: Query<(&Coords, &mut Sprite)>,
) {
    let particle_grid = particle_grid.single();
    for (coords, mut sprite) in coords.iter_mut() {
        sprite.color = get_color(particle_grid.get(*coords));
    }
}

const MAX_HEAT: Scalar = N_PIXELS.y as Scalar * GasProperties::DEFAULT_MASS * -GRAVITY_ACCELERATION.y;

fn get_color(particle: &Particle) -> Color {
    match particle {
        Particle::Vacuum => VACUUM_COLOR,
        Particle::Air { gas_properties } => {
            let temp_param = sigmoid(gas_properties.temperature() / (GasProperties::DEFAULT_TEMPERATURE + MAX_HEAT / (gas_properties.mass * GasProperties::SPECIFIC_HEAT)) - 0.5);
            Color::rgba(
                temp_param,
                1.0 - temp_param,
                1.0 - temp_param,
                gas_properties.mass / GasProperties::DEFAULT_MASS,
            )
        },
    }
}

fn sigmoid(x: f32) -> f32 {
    (x.tanh() + 1.0) / 2.0
}