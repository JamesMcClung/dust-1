use bevy::prelude::*;

use super::{Coords, Particle, PropertyGrid, RelCoords};
use super::types::{Scalar, Vector};
use crate::schedule::SimSet;
use super::path;

pub struct GasPlugin;

impl Plugin for GasPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (gas_bulk_flow, gas_dispersion).chain().in_set(SimSet::Gas));
    }
}

pub struct GasProperties {
    pub mass: Scalar,
    pub momentum: Vector,
    internal_position: Vector,
}

impl Default for GasProperties {
    fn default() -> Self {
        Self {
            mass: Self::DEFAULT_MASS,
            momentum: Vector::ZERO,
            internal_position: Vector::new(0.5, 0.5),
        }
    }
}

impl GasProperties {
    pub const DEFAULT_MASS: Scalar = 100.0;

    const DISPERSION_RATE: f32 = 1.0;

    fn velocity(&self) -> Vector {
        self.momentum / self.mass
    }

    fn merge(&mut self, other: Self) {
        self.momentum += other.momentum;
        self.internal_position = (self.internal_position * self.mass + other.internal_position * other.mass) / (self.mass + other.mass);
        self.mass += other.mass;
    }
}

const MINIMUM_DISPERSION_MASS: Scalar = 1e-3;

/// Air disperses to orthogonally adjacent `Vacuum` and `Air` cells.
/// 
/// The rate of dispersion is determined by `DISPERSION_RATE`, with 0.0 corresponding to no dispersion and 1.0 corresponding to complete dispersion,
/// i.e., a cell of gas will evenly spread itself out across itself and its neighbors in a single tick.
///
/// Air will not disperse if its mass is less than `MINIMUM_DISPERSION_MASS`.
fn gas_dispersion(mut particles: Query<&mut PropertyGrid<Particle>>) {
    let mut particles = particles.single_mut();

    let mut mass_deltas = PropertyGrid::<Scalar>::zero();
    let mut momentum_deltas = PropertyGrid::<Vector>::zero();
    let neighbor_deltas = [RelCoords::new(-1, 0), RelCoords::new(1, 0), RelCoords::new(0, -1), RelCoords::new(0, 1)];
    let max_recipients = neighbor_deltas.len() as Scalar + 1.0;

    for coords in particles.coords() {
        if let Particle::Air { gas_properties } = particles.get(coords) {
            if gas_properties.mass < MINIMUM_DISPERSION_MASS {
                continue;
            }
            
            let mut n_neighbor_recipients = 0.0;
            for delta in neighbor_deltas {
                if matches!(particles.try_get(coords + delta), Some(Particle::Air {..} | Particle::Vacuum)) {
                    n_neighbor_recipients += 1.0;
                }
            }

            let dispersed_mass = gas_properties.mass * GasProperties::DISPERSION_RATE;
            let dispersed_momentum = gas_properties.momentum * GasProperties::DISPERSION_RATE;

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
                        ..default()
                    },
                };
            }
        }
    }
}

fn gas_bulk_flow(mut particles: Query<&mut PropertyGrid<Particle>>) {
    let mut particles = particles.single_mut();
    let mut moved_gases = Vec::<(Coords, GasProperties)>::new();
    
    for coords in particles.coords() {
        if let Particle::Air { gas_properties } = particles.get(coords) {
            let velocity = gas_properties.velocity();
            let new_pos = gas_properties.internal_position + velocity;

            if 0.0 <= new_pos.x && new_pos.x < 1.0
            && 0.0 <= new_pos.y && new_pos.y < 1.0 {
                let Particle::Air { gas_properties } = particles.get_mut(coords) else { panic!() };
                gas_properties.internal_position = new_pos;
                continue;
            }

            let mut net_reflect = RelCoords::new(1, 1);
            let mut end_coords = coords;

            for delta in path::get_path_deltas(gas_properties.internal_position, new_pos) {
                let delta = delta * net_reflect;
                let next_coords = coords + delta;

                match particles.try_get(next_coords) {
                    None => {
                        let reflect = RelCoords::ONE - 2 * delta.abs();
                        net_reflect *= reflect;
                    },
                    Some(Particle::Vacuum | Particle::Air {..}) => {
                        end_coords = next_coords.try_into().unwrap()
                    },
                }
            }

            let Particle::Air { mut gas_properties } = particles.swap(coords, Particle::Vacuum) else { panic!() };
            gas_properties.momentum *= net_reflect;
            if net_reflect.x < 0 {
                gas_properties.internal_position.x = 1.0 - gas_properties.internal_position.x;
            }
            if net_reflect.y < 0 {
                gas_properties.internal_position.y = 1.0 - gas_properties.internal_position.y;
            }
            gas_properties.internal_position += velocity * net_reflect;
            gas_properties.internal_position = gas_properties.internal_position.fract(); // note Vec2::fract behaves differently from f32::fract
            moved_gases.push((end_coords, gas_properties));
        }
    }

    for (coords, moved_gas_properties) in moved_gases {
        match particles.get_mut(coords) {
            p @ Particle::Vacuum => *p = Particle::Air { gas_properties: moved_gas_properties },
            Particle::Air { gas_properties } => gas_properties.merge(moved_gas_properties),
        }
    }
}
