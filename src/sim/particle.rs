use bevy::prelude::Component;

use super::PhysicalProperties;

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

    pub fn collide(&mut self, other: &mut Self) {
        // todo!()
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