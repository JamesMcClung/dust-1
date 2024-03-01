use crate::zero::Zero;

pub type Scalar = f32;
pub type Vector = bevy::prelude::Vec2;

impl Zero for Scalar {
    fn zero() -> Self {
        0.0
    }
}

impl Zero for Vector {
    fn zero() -> Self {
        Self::ZERO
    }
}