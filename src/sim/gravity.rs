use bevy::prelude::*;

use crate::schedule::SimSet;
use super::{types::Vector, Particle, PropertyGrid};


pub const GRAVITY_ACCELERATION: Vector = Vector::new(0.0, -0.01);

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_gravity.in_set(SimSet::Gravity));
    }
}

fn apply_gravity(mut particles: Query<&mut PropertyGrid<Particle>>) {
    let mut particles = particles.single_mut();
    for coords in particles.coords() {
        match particles.get_mut(coords) {
            Particle::Vacuum => {},
            Particle::Air { gas_properties } => gas_properties.apply_impulse(GRAVITY_ACCELERATION * gas_properties.mass),
            Particle::Water { liquid_properties } => liquid_properties.apply_impulse(GRAVITY_ACCELERATION * liquid_properties.mass),
        }
    }
}