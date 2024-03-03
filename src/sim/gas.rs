mod gas_properties;

use bevy::prelude::*;

use super::{Coords, Particle, PropertyGrid, RelCoords};
use super::types::{Scalar, Vector};
use crate::schedule::SimSet;
use crate::zero::Zero;
use super::path;

pub use gas_properties::GasProperties;

pub struct GasPlugin;

impl Plugin for GasPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (gas_bulk_flow, gas_dispersion).chain().in_set(SimSet::Gas));
    }
}


const MINIMUM_DISPERSION_MASS: Scalar = 1e-3;

/// Air disperses to orthogonally adjacent `Vacuum` and `Air` cells.
/// 
/// The rate of dispersion is determined by `DISPERSION_RATE`, with 0.0 corresponding to no dispersion and 1.0 corresponding to complete dispersion,
/// i.e., a cell of gas will evenly spread itself out across itself and its neighbors in a single tick.
/// 
/// Dispersion conserves mass, momentum, and total energy, converting some heat to kinetic energy.
///
/// Air will not disperse if its mass is less than `MINIMUM_DISPERSION_MASS`.
fn gas_dispersion(mut particles: Query<&mut PropertyGrid<Particle>>) {
    let mut particles = particles.single_mut();

    let mut prop_deltas = PropertyGrid::<GasProperties>::zero();
    let dirs = [RelCoords::new(-1, 0), RelCoords::new(1, 0), RelCoords::new(0, -1), RelCoords::new(0, 1)];

    for coords in particles.coords() {
        if let Particle::Air { gas_properties } = particles.get(coords) {
            if gas_properties.mass < MINIMUM_DISPERSION_MASS {
                continue;
            }

            let mut neighbor_dirs = vec![];
            for dir in dirs {
                match particles.try_get(coords + dir) {
                    Some(
                        | Particle::Vacuum
                        | Particle::Air { .. }
                    ) => neighbor_dirs.push(dir),
                    _ => ()
                }
            }
            
            let Particle::Air { gas_properties } = particles.get_mut(coords) else { panic!() };
            let dispersed_props = gas_properties.disperse(neighbor_dirs.iter().map(|dir| Vector::from(*dir)).collect());
            for (dir, props) in std::iter::zip(neighbor_dirs, dispersed_props) {
                prop_deltas.try_get_mut(coords + dir).unwrap().merge(props);
            }
        }
    }

    for coords in particles.coords() {
        let prop_deltas = prop_deltas.get(coords);
        if prop_deltas.mass != 0.0 {
            match particles.get_mut(coords) {
                Particle::Air { gas_properties } => gas_properties.merge(*prop_deltas),
                p @ Particle::Vacuum => *p = Particle::Air {
                    gas_properties: *prop_deltas,
                },
                _ => (),
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
                    None | Some(Particle::Water {..}) => {
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
            _ => panic!(),
        }
    }
}
