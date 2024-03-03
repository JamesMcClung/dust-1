use bevy::prelude::Component;

use super::gas::GasProperties;

#[derive(Clone, Copy, Component)]
pub enum Particle {
    Vacuum,
    Air {
        gas_properties: GasProperties,
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
        }
    }
}

pub mod names {
    pub const VACUUM: &'static str = "Vacuum";
    pub const AIR: &'static str = "Air";
}