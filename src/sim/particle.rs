use bevy::prelude::Component;

use super::{types::Vector, PhysicalProperties};

#[derive(Clone, Copy, Component)]
pub enum Particle {
    Vacuum,
    Air {
        gas_properties: PhysicalProperties,
    },
    Water {
        liquid_properties: PhysicalProperties,
    },
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
        }
    }

    pub fn get_physical_properties_mut(&mut self) -> Option<&mut PhysicalProperties> {
        match self {
            Self::Air { gas_properties } => Some(gas_properties),
            Self::Water { liquid_properties } => Some(liquid_properties),
            _ => None,
        }
    }

    pub fn collide(&mut self, other: &mut Self, delta_cell: Vector) {
        if let (Some(properties_1), Some(properties_2)) = (self.get_physical_properties_mut(), other.get_physical_properties_mut()) {
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
}

pub mod defualts {
    use super::Particle;
    use crate::sim::physical_properties::defaults;

    pub const VACUUM: Particle = Particle::Vacuum;
    pub const AIR: Particle = Particle::Air { gas_properties: defaults::AIR };
    pub const WATER: Particle = Particle::Water { liquid_properties: defaults::WATER };
}