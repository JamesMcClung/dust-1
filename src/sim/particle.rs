use super::gas::GasProperties;

#[derive(Clone, Copy)]
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
            Self::Vacuum => "Vacuum",
            Self::Air { .. } => "Air",
        }
    }
}