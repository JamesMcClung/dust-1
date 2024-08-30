use super::PhysicalProperties;

pub const AIR: PhysicalProperties = PhysicalProperties::new(masses::AIR, temperatures::AIR, specific_heats::AIR);
pub const WATER: PhysicalProperties = PhysicalProperties::new(masses::WATER, temperatures::WATER, specific_heats::WATER);

mod specific_heats {
    use crate::sim::types::Scalar;

    pub const AIR: Scalar = 1e-3;
    pub const WATER: Scalar = 1.0;
}

mod masses {
    use crate::sim::types::Scalar;

    pub const AIR: Scalar = 1.0;
    pub const WATER: Scalar = 100.0;
}

mod temperatures {
    use crate::sim::types::Scalar;

    pub const NORMAL: Scalar = 1.0;

    pub const AIR: Scalar = NORMAL;
    pub const WATER: Scalar = NORMAL;
}
