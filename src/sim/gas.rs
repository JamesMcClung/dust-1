use bevy::prelude::*;

use super::{Particle, PropertyGrid, RelCoords};
use super::types::{Scalar, Vector};
use crate::schedule::SimSet;

pub struct GasPlugin;

impl Plugin for GasPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, gas_dispersion.in_set(SimSet::Physics));
    }
}

pub struct GasProperties {
    pub mass: Scalar,
    pub momentum: Vector,
}

impl Default for GasProperties {
    fn default() -> Self {
        Self {
            mass: NORMAL_GAS_DENSITY,
            momentum: Vector::ZERO,
        }
    }
}

pub const NORMAL_GAS_DENSITY: Scalar = 100.0;

const DISPERSION_RATE: f32 = 1.0;

/**
    Air disperses to orthogonally adjacent `Vacuum` and `Air` cells.
    
    The rate of dispersion is determined by `DISPERSION_RATE`, with 0.0 corresponding to no dispersion and 1.0 corresponding to complete dispersion,
    i.e., a cell of gas will evenly spread itself out across itself and its neighbors in a single tick.
*/
fn gas_dispersion(mut particles: Query<&mut PropertyGrid<Particle>>) {
    let mut particles = particles.single_mut();

    let mut mass_deltas = PropertyGrid::<Scalar>::zero();
    let mut momentum_deltas = PropertyGrid::<Vector>::zero();
    let neighbor_deltas = [RelCoords::new(-1, 0), RelCoords::new(1, 0), RelCoords::new(0, -1), RelCoords::new(0, 1)];
    let max_recipients = neighbor_deltas.len() as Scalar + 1.0;

    for coords in particles.coords() {
        if let Particle::Air { gas_properties } = particles.get(coords) {
            let mut n_neighbor_recipients = 0.0;
            for delta in neighbor_deltas {
                if matches!(particles.try_get(coords + delta), Some(Particle::Air {..} | Particle::Vacuum)) {
                    n_neighbor_recipients += 1.0;
                }
            }

            let dispersed_mass = gas_properties.mass * DISPERSION_RATE;
            let dispersed_momentum = gas_properties.momentum * DISPERSION_RATE;

            *mass_deltas.get_mut(coords) -= dispersed_mass * n_neighbor_recipients / max_recipients;
            *momentum_deltas.get_mut(coords) -= dispersed_momentum * n_neighbor_recipients / max_recipients;
            
            for delta in neighbor_deltas {
                let neighbor = coords + delta;
                if matches!(particles.try_get(neighbor), Some(Particle::Air {..} | Particle::Vacuum)) {
                    *mass_deltas.try_get_mut(neighbor).unwrap() += dispersed_mass / max_recipients;
                    *momentum_deltas.try_get_mut(neighbor).unwrap() += dispersed_momentum / max_recipients;
                }
            }
        }
    }

    for coords in particles.coords() {
        if *mass_deltas.get(coords) != 0.0 {
            if let Particle::Air { gas_properties } = particles.get_mut(coords) {
                gas_properties.mass += mass_deltas.get(coords);
                gas_properties.momentum += *momentum_deltas.get(coords);
            } else {
                *particles.get_mut(coords) = Particle::Air {
                    gas_properties: GasProperties {
                        mass: *mass_deltas.get(coords),
                        momentum: *momentum_deltas.get(coords),
                    },
                };
            }
        }
    }
    
}
