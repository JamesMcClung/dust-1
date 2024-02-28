use super::gas::GasDensityType;

pub enum Particle {
    Vacuum,
    Air {
        density: GasDensityType,
    },
}

impl Default for Particle {
    fn default() -> Self {
        Self::Vacuum
    }
}
