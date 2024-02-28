use bevy::prelude::Color;

static VACUUM_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);
static AIR_COLOR: Color = Color::rgba(0.0, 0.9, 0.9, 0.2);

pub enum Particle {
    Vacuum,
    Air,
}

impl Particle {
    pub fn get_color(&self) -> Color {
        match self {
            Self::Vacuum => VACUUM_COLOR,
            Self::Air => AIR_COLOR,
        }
    }
}

impl Default for Particle {
    fn default() -> Self {
        Self::Vacuum
    }
}
