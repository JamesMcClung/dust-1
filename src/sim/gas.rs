use bevy::prelude::*;

use super::{Particle, PropertyGrid, N_PIXELS};
use super::types::Scalar;
use crate::schedule::SimSet;

pub struct GasPlugin;

impl Plugin for GasPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, gas_dispersion.in_set(SimSet::Physics));
    }
}

pub struct GasProperties {
    pub density: Scalar,
}

impl Default for GasProperties {
    fn default() -> Self {
        Self {
            density: NORMAL_GAS_DENSITY,
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

    let mut density_deltas = [[0.0 as Scalar; N_PIXELS.y]; N_PIXELS.x];
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

                let dispersed_amount = gas_properties.density * DISPERSION_RATE;
                density_deltas[i as usize][j as usize] -= dispersed_amount * n_neighbor_recipients / max_recipients;
                for (ni, nj) in neighbors {
                    if matches!(particles.get_checked(i + ni, j + nj), Some(Particle::Air {..} | Particle::Vacuum)) {
                        density_deltas[(i + ni) as usize][(j + nj) as usize] += dispersed_amount / max_recipients;
                    }
                }
            }
        }
    }

    for x in 0..N_PIXELS.x {
        for y in 0..N_PIXELS.y {
            if density_deltas[x][y] != 0.0 {
                if let Particle::Air { ref mut gas_properties } = *particles.get_mut(x, y) {
                    gas_properties.density += density_deltas[x][y];
                } else {
                    *particles.get_mut(x, y) = Particle::Air {
                        gas_properties: GasProperties {
                            density: density_deltas[x][y],
                        },
                    };
                }
            }
        }
    }
}
