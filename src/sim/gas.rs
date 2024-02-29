use std::array;

use bevy::prelude::*;

use super::{Particle, PropertyGrid, N_PIXELS};
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

    let mut mass_deltas = [[0.0 as Scalar; N_PIXELS.y]; N_PIXELS.x];
    let mut momentum_deltas: [[Vector; N_PIXELS.y]; N_PIXELS.x] = array::from_fn(|_| array::from_fn(|_| Vector::ZERO));
    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let max_recipients = neighbors.len() as Scalar + 1.0;

    for i in 0..N_PIXELS.x as isize {
        for j in 0..N_PIXELS.y as isize {
            if let Particle::Air { gas_properties } = particles.get(i as usize, j as usize) {
                let mut n_neighbor_recipients: Scalar = 0.0;
                for (ni, nj) in neighbors {
                    if matches!(particles.get_checked(i + ni, j + nj), Some(Particle::Air {..} | Particle::Vacuum)) {
                        n_neighbor_recipients += 1.0;
                    }
                }

                let dispersed_mass = gas_properties.mass * DISPERSION_RATE;
                let dispersed_momentum = gas_properties.momentum * DISPERSION_RATE;

                mass_deltas[i as usize][j as usize] -= dispersed_mass * n_neighbor_recipients / max_recipients;
                momentum_deltas[i as usize][j as usize] -= dispersed_momentum * n_neighbor_recipients / max_recipients;
                
                for (ni, nj) in neighbors {
                    if matches!(particles.get_checked(i + ni, j + nj), Some(Particle::Air {..} | Particle::Vacuum)) {
                        mass_deltas[(i + ni) as usize][(j + nj) as usize] += dispersed_mass / max_recipients;
                        momentum_deltas[(i + ni) as usize][(j + nj) as usize] += dispersed_momentum / max_recipients;
                    }
                }
            }
        }
    }

    for x in 0..N_PIXELS.x {
        for y in 0..N_PIXELS.y {
            if mass_deltas[x][y] != 0.0 {
                if let Particle::Air { ref mut gas_properties } = *particles.get_mut(x, y) {
                    gas_properties.mass += mass_deltas[x][y];
                    gas_properties.momentum += momentum_deltas[x][y];
                } else {
                    *particles.get_mut(x, y) = Particle::Air {
                        gas_properties: GasProperties {
                            mass: mass_deltas[x][y],
                            momentum: momentum_deltas[x][y],
                        },
                    };
                }
            }
        }
    }
}
