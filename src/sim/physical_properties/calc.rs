use const_soft_float::soft_f32::SoftF32;

use crate::sim::types::{Scalar, Vector};

pub fn temperature(heat: Scalar, mass: Scalar, specific_heat: Scalar) -> Scalar {
    heat / (mass * specific_heat)
}
pub const fn temperature_const(heat: Scalar, mass: Scalar, specific_heat: Scalar) -> Scalar {
    SoftF32(heat).div(SoftF32(mass).mul(SoftF32(specific_heat))).to_f32()
}

#[cfg(test)] // remove this when it is used elsewhere, but for now, it is used to test heat_const
pub fn heat(temperature: Scalar, mass: Scalar, specific_heat: Scalar) -> Scalar {
    temperature * mass * specific_heat
}
pub const fn heat_const(temperature: Scalar, mass: Scalar, specific_heat: Scalar) -> Scalar {
    SoftF32(temperature).mul(SoftF32(mass)).mul(SoftF32(specific_heat)).to_f32()
}

pub fn velocity(momentum: Vector, mass: Scalar) -> Vector {
    momentum / mass
}

pub fn kinetic_energy(momentum: Vector, mass: Scalar) -> Scalar {
    if mass == 0.0 {
        0.0
    } else {
        momentum.length_squared() / (2.0 * mass)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::*;

    #[test]
    fn temperature_exprs_agree() {
        let heat = 1.2;
        let mass = 3.71;
        let specific_heat = 0.99;
        assert_f32_near!(temperature(heat, mass, specific_heat), temperature_const(heat, mass, specific_heat));
    }
    
    #[test]
    fn heat_exprs_agree() {
        let temperature = 1.2;
        let mass = 3.71;
        let specific_heat = 0.99;
        assert_f32_near!(heat(temperature, mass, specific_heat), heat_const(temperature, mass, specific_heat));
    }

    #[test]
    fn zero_mass_ke() {
        assert_f32_near!(0.0, kinetic_energy(Vector::ONE, 0.0));
    }
}