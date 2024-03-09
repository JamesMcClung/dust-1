mod wall;

use bevy::prelude::Component;

use super::{types::Vector, PhysicalProperties};
pub use wall::Wall;

#[derive(Clone, Copy, Component)]
pub enum Particle {
    Vacuum,
    Air {
        physical_properties: PhysicalProperties,
    },
    Water {
        physical_properties: PhysicalProperties,
    },
    Wall(Wall),
}

impl Default for Particle {
    fn default() -> Self {
        Self::Vacuum
    }
}

impl Particle {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Vacuum => names::VACUUM,
            Self::Air { .. } => names::AIR,
            Self::Water { .. } => names::WATER,
            Self::Wall(_) => names::WALL,
        }
    }

    pub fn physical_properties_mut(&mut self) -> Option<&mut PhysicalProperties> {
        match self {
            Self::Air { physical_properties } => Some(physical_properties),
            Self::Water { physical_properties } => Some(physical_properties),
            _ => None,
        }
    }

    pub fn collide(&mut self, other: &mut Self, delta_cell: Vector) {
        if let (Some(properties_1), Some(properties_2)) = (self.physical_properties_mut(), other.physical_properties_mut()) {
            let pos_2 = properties_2.internal_position + delta_cell;
            
            let collision_dir = (pos_2 - properties_1.internal_position).normalize();
            let total_mass = properties_1.mass + properties_2.mass;

            let delta_p_1 = (properties_1.mass * properties_2.momentum - properties_2.mass * properties_1.momentum).project_onto(collision_dir) / total_mass;
            let delta_p_2 = (properties_2.mass * properties_1.momentum - properties_1.mass * properties_2.momentum).project_onto(collision_dir) / total_mass;

            properties_1.momentum += delta_p_1;
            properties_2.momentum += delta_p_2;
        }
    }
}

pub mod names {
    pub const VACUUM: &'static str = "Vacuum";
    pub const AIR: &'static str = "Air";
    pub const WATER: &'static str = "Water";

    pub const WALL: &'static str = "Wall";
}

pub mod defualts {
    use super::{Particle, Wall};
    use crate::sim::physical_properties::defaults;

    pub const VACUUM: Particle = Particle::Vacuum;
    pub const AIR: Particle = Particle::Air { physical_properties: defaults::AIR };
    pub const WATER: Particle = Particle::Water { physical_properties: defaults::WATER };

    pub const WALL_REFLECTIVE: Particle = Particle::Wall(Wall::Reflective);
    pub const WALL_ABSORPTIVE: Particle = Particle::Wall(Wall::Absorptive);
}