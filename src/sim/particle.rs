pub enum Particle {
    Vacuum,
    Air,
}

impl Default for Particle {
    fn default() -> Self {
        Self::Vacuum
    }
}
