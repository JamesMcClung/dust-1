use crate::sim::{dir::Dir, PhysicalProperties};

#[derive(Debug, Clone, Copy)]
pub enum Wall {
    Absorptive,
    Reflective,
}

impl Wall {
    pub fn collide(&self, physical_properties: &mut PhysicalProperties, delta_cell: Dir) {
        match (self, delta_cell) {
            (Self::Absorptive, Dir::Left | Dir::Right) => physical_properties.momentum.x = 0.0,
            (Self::Reflective, Dir::Left | Dir::Right) => physical_properties.momentum.x *= -1.0,
            (Self::Absorptive, Dir::Up | Dir::Down) => physical_properties.momentum.y = 0.0,
            (Self::Reflective, Dir::Up | Dir::Down) => physical_properties.momentum.y *= -1.0,
            (_, Dir::Zero) => (),
        }
    }
}