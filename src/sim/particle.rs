use super::gas::GasProperties;

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
