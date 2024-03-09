mod wall;

use bevy::prelude::Component;

use super::{dir::Dir, PhysicalProperties, RelCoords};
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

    pub fn collide(&mut self, other: &mut Self, delta_cell: RelCoords) {
        match (self, other) {
            (
                Self::Air { physical_properties: props_1 } | Self::Water { physical_properties: props_1 },
                Self::Air { physical_properties: props_2 } | Self::Water { physical_properties: props_2 },
            ) => props_1.collide(props_2, delta_cell.into()),
            (
                Self::Wall(wall),
                Self::Air { physical_properties } | Self::Water { physical_properties },
            ) | (
                Self::Air { physical_properties } | Self::Water { physical_properties },
                Self::Wall(wall),
            ) => wall.collide(physical_properties, delta_cell.into()),
            _ => (),
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