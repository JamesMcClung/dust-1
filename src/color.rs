use bevy::prelude::*;

use crate::schedule::SimSet;
use crate::sim::gravity::GRAVITY_ACCELERATION;
use crate::sim::types::Scalar;
use crate::sim::{Coords, Particle, PropertyGrid, N_PIXELS, physical_properties};

pub struct ColorPlugin;

impl Plugin for ColorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_colors.in_set(SimSet::Recolor));
    }
}


fn update_colors(
    particle_grid: Query<&PropertyGrid<Particle>, Changed<PropertyGrid<Particle>>>,
    mut coords: Query<(&Coords, &mut Sprite)>,
) {
    let Ok(particle_grid) = particle_grid.get_single() else {
        return;
    };

    for (coords, mut sprite) in coords.iter_mut() {
        sprite.color = get_color(particle_grid.get(*coords));
    }
}

const KE_FROM_AIR_FALLING: Scalar = N_PIXELS.y as Scalar * physical_properties::defaults::AIR.mass * -GRAVITY_ACCELERATION.y;
const HIGH_AIR_TEMPERATURE: Scalar = physical_properties::calc::temperature_const(
    physical_properties::defaults::AIR.heat + KE_FROM_AIR_FALLING,
    physical_properties::defaults::AIR.mass,
    physical_properties::defaults::AIR.specific_heat
);

pub fn get_color(particle: &Particle) -> Color {
    match particle {
        Particle::Vacuum => Color::rgba(0.0, 0.0, 0.0, 0.0),
        Particle::Air { gas_properties } => {
            let temp_param = sigmoid(gas_properties.temperature() / HIGH_AIR_TEMPERATURE - 0.5);
            Color::rgba(
                temp_param,
                1.0 - temp_param,
                1.0 - temp_param,
                gas_properties.mass / physical_properties::defaults::AIR.mass,
            )
        },
        Particle::Water { .. } => {
            Color::rgba(0.0, 0.8, 0.9, 0.9)
        },
        Particle::Wall(_) => Color::GRAY,
    }
}

fn sigmoid(x: f32) -> f32 {
    (x.tanh() + 1.0) / 2.0
}