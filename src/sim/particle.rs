use bevy::prelude::Color;

static VACUUM_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);

pub enum Particle {
    Vacuum,
}

impl Particle {
    pub fn get_color(&self) -> Color {
        match self {
            Self::Vacuum => VACUUM_COLOR,
        }
    }
}

impl Default for Particle {
    fn default() -> Self {
        Self::Vacuum
    }
}
