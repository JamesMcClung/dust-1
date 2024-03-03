pub mod calc;
pub mod defaults;

use crate::sim::types::{Scalar, Vector};
use crate::sim::MAX_NEIGHBORS;
use crate::zero::Zero;

#[derive(Clone, Copy, Debug)]
pub struct PhysicalProperties {
    pub mass: Scalar,
    pub momentum: Vector,
    pub heat: Scalar,
    pub specific_heat: Scalar,
    pub internal_position: Vector,
}

impl Zero for PhysicalProperties {
    fn zero() -> Self {
        Self {
            mass: 0.0,
            momentum: Vector::ZERO,
            heat: 0.0,
            specific_heat: 0.0,
            internal_position: Vector::ZERO,
        }
    }
}

impl PhysicalProperties {
    pub const fn new(mass: Scalar, temperature: Scalar, specific_heat: Scalar) -> Self {
        Self {
            mass,
            momentum: Vector::ZERO,
            heat: calc::heat_const(temperature, mass, specific_heat),
            specific_heat,
            internal_position: Vector::new(0.5, 0.5),
        }
    }

    pub fn velocity(&self) -> Vector {
        calc::velocity(self.momentum, self.mass)
    }

    pub fn temperature(&self) -> Scalar {
        calc::temperature(self.heat, self.mass, self.specific_heat)
    }

    pub fn kinetic_energy(&self) -> Scalar {
        calc::kinetic_energy(self.momentum, self.mass)
    }

    pub fn merge(&mut self, other: Self) {
        if self.mass == 0.0 {
            *self = other;
            return;
        }
        if other.mass == 0.0 {
            return;
        }
        
        self.internal_position = (self.internal_position * self.mass + other.internal_position * other.mass) / (self.mass + other.mass);

        let ke_before = self.kinetic_energy() + other.kinetic_energy();
        self.momentum += other.momentum;
        self.mass += other.mass;
        let ke_after = self.kinetic_energy();
        
        // it can be shown that ke_before >= ke_after, provided both masses >= 0
        self.heat += other.heat + (ke_before - ke_after);
    }


    const DISPERSION_RATE: f32 = 1.0; // between 0 and 1, inclusive

    // from an arcane derivation
    const BOOST_PARAMETER: Scalar = 0.9; // heat is guaranteed to be positive when this is strictly less than 1
    const BOOST_CONSTANT: Scalar = 2.0 / 14.8323969742; // f32::sqrt((MAX_NEIGHBORS * (MAX_NEIGHBORS + 1) * (2 * MAX_NEIGHBORS + 3)) as f32)

    pub fn disperse(&mut self, dirs: Vec<Vector>) -> Vec<Self> {
        let dispersed_fraction_per_dir = Self::DISPERSION_RATE / (MAX_NEIGHBORS as f32 + 1.0);
        let n_neighbors = dirs.len() as f32;
        
        let other_mass_after = self.mass * dispersed_fraction_per_dir;
        let my_mass_after = self.mass - n_neighbors * other_mass_after;
        
        let abs_momentum_boost = Self::BOOST_PARAMETER * Self::BOOST_CONSTANT * Scalar::sqrt(Self::DISPERSION_RATE * self.mass * self.heat);
        let other_momenta_after = dirs.iter().map(|dir| *dir * abs_momentum_boost + self.momentum * dispersed_fraction_per_dir).collect::<Vec<_>>();
        let my_momentum_after = self.momentum - other_momenta_after.iter().sum::<Vector>();
        
        let calc_kinetic_energy = |momentum: Vector, mass: Scalar| momentum.length_squared() / (2.0 * mass);
        let ke_before = self.kinetic_energy();
        let ke_after = calc_kinetic_energy(my_momentum_after, my_mass_after) + other_momenta_after.iter().map(|momentum| calc_kinetic_energy(*momentum, other_mass_after)).sum::<Scalar>();
        let total_heat_after = self.heat + ke_before - ke_after;

        let other_heat_after = total_heat_after * dispersed_fraction_per_dir;
        let my_heat_after = total_heat_after - n_neighbors * other_heat_after;

        self.mass = my_mass_after;
        self.momentum = my_momentum_after;
        self.heat = my_heat_after;

        other_momenta_after.into_iter().map(|other_momentum_after| PhysicalProperties {
            mass: other_mass_after,
            momentum: other_momentum_after,
            heat: other_heat_after,
            internal_position: self.internal_position,
            specific_heat: self.specific_heat,
        }).collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::*;
    
    // make sure test properties can be evenly divided
    const TEST_MASS: Scalar = 3.0 * 4.0 * 5.0;
    const TEST_MOMENTUM: Vector = Vector::new(3.0 * 4.0 * 5.0, 0.0);
    const TEST_HEAT: Scalar = 3.0 * 4.0 * 5.0;
    const TEST_INTERNAL_POSITION: Vector = Vector::new(0.5, 0.5);
    const TEST_SPECIFIC_HEAT: Scalar = 1.0;

    fn get_test_properties() -> PhysicalProperties {
        PhysicalProperties {
            mass: TEST_MASS,
            momentum: TEST_MOMENTUM,
            heat: TEST_HEAT,
            internal_position: TEST_INTERNAL_POSITION,
            specific_heat: TEST_SPECIFIC_HEAT,
        }
    }

    #[test]
    fn merge_identical() {
        let mut a = get_test_properties();
        let b = get_test_properties();

        let total_mass_before = a.mass + b.mass;
        let total_momentum_before = a.momentum + b.momentum;
        let total_heat_before = a.heat + b.heat;

        a.merge(b);

        assert_f32_near!(a.mass, total_mass_before);
        assert_f32_near!(a.momentum.x, total_momentum_before.x);
        assert_f32_near!(a.momentum.y, total_momentum_before.y);
        assert_f32_near!(a.heat, total_heat_before);
    }

    #[test]
    fn merge_opposite() {
        let mut a = get_test_properties();
        let mut b = get_test_properties();
        b.momentum = -b.momentum;

        let total_mass_before = a.mass + b.mass;
        let total_momentum_before = a.momentum + b.momentum;
        let total_heat_before = a.heat + b.heat;
        let total_ke_before = a.kinetic_energy() + b.kinetic_energy();

        a.merge(b);

        assert_f32_near!(a.mass, total_mass_before);
        assert_f32_near!(a.momentum.x, total_momentum_before.x);
        assert_f32_near!(a.momentum.y, total_momentum_before.y);
        assert_f32_near!(a.heat + a.kinetic_energy(), total_heat_before + total_ke_before);
    }

    #[test]
    fn merge_orthogonal() {
        let mut a = get_test_properties();
        let mut b = get_test_properties();
        b.momentum = Vector::new(-b.momentum.y, b.momentum.x);

        let total_mass_before = a.mass + b.mass;
        let total_momentum_before = a.momentum + b.momentum;
        let total_heat_before = a.heat + b.heat;
        let total_ke_before = a.kinetic_energy() + b.kinetic_energy();

        a.merge(b);

        assert_f32_near!(a.mass, total_mass_before);
        assert_f32_near!(a.momentum.x, total_momentum_before.x);
        assert_f32_near!(a.momentum.y, total_momentum_before.y);
        assert_f32_near!(a.heat + a.kinetic_energy(), total_heat_before + total_ke_before);
    }

    #[test]
    fn merge_smaller() {
        let mut a = get_test_properties();
        let mut b = get_test_properties();
        b.mass /= 2.0;
        b.momentum /= 3.0;
        b.heat /= 4.0;

        let total_mass_before = a.mass + b.mass;
        let total_momentum_before = a.momentum + b.momentum;
        let total_heat_before = a.heat + b.heat;
        let total_ke_before = a.kinetic_energy() + b.kinetic_energy();

        a.merge(b);

        assert_f32_near!(a.mass, total_mass_before);
        assert_f32_near!(a.momentum.x, total_momentum_before.x);
        assert_f32_near!(a.momentum.y, total_momentum_before.y);
        assert_f32_near!(a.heat + a.kinetic_energy(), total_heat_before + total_ke_before);
    }

    #[test]
    fn merge_into_zero() {
        let mut a = PhysicalProperties::zero();
        let b = get_test_properties();

        a.merge(b);

        assert_f32_near!(a.mass, TEST_MASS);
        assert_f32_near!(a.momentum.x, TEST_MOMENTUM.x);
        assert_f32_near!(a.momentum.y, TEST_MOMENTUM.y);
        assert_f32_near!(a.heat, TEST_HEAT);
        assert_f32_near!(a.internal_position.x, TEST_INTERNAL_POSITION.x);
        assert_f32_near!(a.internal_position.y, TEST_INTERNAL_POSITION.y);
    }

    #[test]
    fn merge_from_zero() {
        let mut a = get_test_properties();
        let b = PhysicalProperties::zero();

        a.merge(b);

        assert_f32_near!(a.mass, TEST_MASS);
        assert_f32_near!(a.momentum.x, TEST_MOMENTUM.x);
        assert_f32_near!(a.momentum.y, TEST_MOMENTUM.y);
        assert_f32_near!(a.heat, TEST_HEAT);
        assert_f32_near!(a.internal_position.x, TEST_INTERNAL_POSITION.x);
        assert_f32_near!(a.internal_position.y, TEST_INTERNAL_POSITION.y);
    }



    fn disperse_4_ways(gas_properties: &mut PhysicalProperties) -> Vec<PhysicalProperties> {
        let dirs = [Vector::new(1.0, 0.0), Vector::new(0.0, 1.0), Vector::new(-1.0, 0.0), Vector::new(0.0, -1.0)];
        gas_properties.disperse(dirs.into())
    }

    #[test]
    fn disperse_4_way_mass() {
        let mut original = get_test_properties();
        let total_mass_before = original.mass;
        
        let disperseds = disperse_4_ways(&mut original);
        let total_mass_after = original.mass + disperseds.iter().map(|dispersed| dispersed.mass).sum::<Scalar>();
        
        assert_f32_near!(total_mass_before, total_mass_after);
        
        assert_f32_near!(original.mass, total_mass_after / 5.0);
        for dispersed in disperseds {
            assert_f32_near!(dispersed.mass, total_mass_after / 5.0);
        }
    }

    #[test]
    fn disperse_4_way_momentum() {
        let mut original = get_test_properties();
        let total_momentum_before = original.momentum;
        
        let disperseds = disperse_4_ways(&mut original);
        let total_momentum_after = original.momentum + disperseds.iter().map(|dispersed| dispersed.momentum).sum::<Vector>();
        
        assert_f32_near!(total_momentum_before.x, total_momentum_after.x);
        assert_f32_near!(total_momentum_before.y, total_momentum_after.y);

        assert_f32_near!(original.momentum.x, total_momentum_before.x / 5.0);
        assert_f32_near!(original.momentum.y, total_momentum_before.y / 5.0);

        assert_f32_near!(disperseds[0].momentum.y, disperseds[2].momentum.y);
        assert_f32_near!(disperseds[0].momentum.x + disperseds[2].momentum.x, 2.0 * original.momentum.x);
        
        assert_f32_near!(disperseds[1].momentum.x, disperseds[3].momentum.x);
        assert_f32_near!(disperseds[1].momentum.y + disperseds[3].momentum.y, 2.0 * original.momentum.y);
    }

    #[test]
    fn disperse_4_way_heat() {
        let mut original = get_test_properties();
        let heat_before = original.heat;
        let ke_before = original.kinetic_energy();

        let disperseds = disperse_4_ways(&mut original);
        let heat_after = original.heat + disperseds.iter().map(|dispersed| dispersed.heat).sum::<Scalar>();
        let ke_after = original.kinetic_energy() + disperseds.iter().map(PhysicalProperties::kinetic_energy).sum::<Scalar>();
        
        assert_f32_near!(heat_before + ke_before, heat_after + ke_after);

        assert_f32_near!(original.heat, heat_after / 5.0);
        for dispersed in disperseds {
            assert_f32_near!(dispersed.heat, heat_after / 5.0);
        }
    }
}