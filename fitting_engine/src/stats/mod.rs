use num_traits::{AsPrimitive, NumOps};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::ops::Deref;

pub mod capacitor;
pub mod defense;
pub mod drone;
pub mod fitting;
pub mod movement;
pub mod sensor;

pub trait Stat<Input> {
    fn apply(&self, stat_mods: Vec<&Input>) -> Self;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModificationType<T>
where
    T: NumOps + PartialEq + PartialOrd,
{
    Multiplicative(T),
    Additive(T),
    FittingCost(T),
}

impl<T> ModificationType<T>
where
    T: NumOps + PartialEq + PartialOrd,
{
    pub fn additive(&self) -> bool {
        match self {
            Self::Additive(_) => true,
            Self::Multiplicative(_) => false,
            Self::FittingCost(_) => false,
        }
    }

    pub fn multiplicative(&self) -> bool {
        match self {
            Self::Additive(_) => false,
            Self::Multiplicative(_) => true,
            Self::FittingCost(_) => false,
        }
    }

    pub fn fitting_cost(&self) -> bool {
        match self {
            Self::Multiplicative(_) => false,
            Self::Additive(_) => false,
            Self::FittingCost(_) => true,
        }
    }

    pub fn apply<'a, V: AsPrimitive<T>>(&self, val: V) -> V
    where
        T: Copy,
        T: Clone,
        T: 'a,
        T: AsPrimitive<V>,
    {
        match self {
            ModificationType::Multiplicative(x) => val.as_().mul(*x),
            ModificationType::Additive(x) => val.as_().add(*x),
            ModificationType::FittingCost(x) => val.as_().sub(*x),
        }
        .as_()
    }
}

impl Default for ModificationType<u8> {
    fn default() -> Self {
        Self::Additive(0u8)
    }
}

impl<T> PartialEq for &ModificationType<T>
where
    T: NumOps + PartialEq + PartialOrd,
{
    fn eq(&self, other: &Self) -> bool {
        self.additive() == other.additive() && self.deref().eq(other)
    }
}

impl<T> PartialOrd for &ModificationType<T>
where
    T: NumOps + PartialEq + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let s: &T = self.deref();
        s.partial_cmp(other.deref())
    }
}

impl<T> Deref for ModificationType<T>
where
    T: NumOps + PartialEq + PartialOrd,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            ModificationType::Multiplicative(x) => x,
            ModificationType::Additive(x) => x,
            ModificationType::FittingCost(x) => x,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::stats::fitting::Fitting;
    use crate::stats::sensor::*;
    use once_cell::sync::Lazy;

    macro_rules! assert_partial_eq {
        ($expected:ident, $actual:ident) => {
            assert!($expected.eq(&$actual))
        };
    }

    pub static SENSOR_STATS: Lazy<Sensor> = Lazy::new(|| Sensor::new(50.0, 200, 32.0, 5));
    pub static FITTING_STATS: Lazy<Fitting> = Lazy::new(|| Fitting::new(250.0, 250.0, 400, 375.0));
    mod stat_modifications_are_of_the_types {
        use crate::stats::fitting::*;
        use crate::stats::sensor::*;
        use crate::stats::tests::{FITTING_STATS, SENSOR_STATS};
        use crate::stats::ModificationType;
        use crate::stats::Stat;

        #[test]
        fn additive() {
            let modification = SensorModifications::new(
                ModificationType::default(),
                ModificationType::Additive(50),
                ModificationType::default(),
                ModificationType::default(),
            );
            let expected = Sensor::new(50.0, 250, 32.0, 5);
            let actual = SENSOR_STATS.apply(vec![&modification]);
            assert_partial_eq!(expected, actual);
        }

        #[test]
        fn multiplicative() {
            let modification = SensorModifications::new(
                ModificationType::default(),
                ModificationType::Multiplicative(1.2),
                ModificationType::default(),
                ModificationType::default(),
            );
            let expected = Sensor::new(50.0, 240, 32.0, 5);
            let actual = SENSOR_STATS.apply(vec![&modification]);
            assert_partial_eq!(expected, actual);
        }

        #[test]
        fn subtractive_fitting_costs() {
            let modification = FittingModifications::new(
                ModificationType::FittingCost(50.0),
                ModificationType::default(),
                ModificationType::default(),
                ModificationType::default(),
            );
            let expected = Fitting::new(200.0, 250.0, 400, 375.0);
            let actual = FITTING_STATS.apply(vec![&modification]);
            //TODO:
            //calibration    400 => 144
            //cargo          375 => 255
            //?????
            println!(
                "cpu: {}, pg: {}, calibration:{}, cargo:{}",
                actual.cpu, actual.pg, actual.calibration, actual.cargo
            );
            assert_partial_eq!(expected, actual);
        }
    }
    mod stat_modifications_are_applied {
        use crate::stats::fitting::*;
        use crate::stats::sensor::*;
        use crate::stats::tests::{FITTING_STATS, SENSOR_STATS};
        use crate::stats::ModificationType;
        use crate::stats::Stat;

        #[test]
        fn additive_before_multiplicative() {
            let mod_add = SensorModifications::new(
                ModificationType::Additive(50),
                ModificationType::default(),
                ModificationType::default(),
                ModificationType::default(),
            );
            let mod_multi = SensorModifications::new(
                ModificationType::Multiplicative(2),
                ModificationType::default(),
                ModificationType::default(),
                ModificationType::default(),
            );
            let expected = Sensor::new(200.0, 200, 32.0, 5);
            let actual = SENSOR_STATS.apply(vec![&mod_multi, &mod_add]);
            println!(
                "targeting range {}, scan res: {}, sensor strength: {}, max targets: {}",
                actual.targeting_range,
                actual.scan_res,
                actual.sensor_strength,
                actual.max_locked_targets
            );
            assert_partial_eq!(expected, actual);
        }

        #[test]
        fn multiplicative_before_fitting() {
            let mod_multi = FittingModifications::new(
                ModificationType::Multiplicative(1.2),
                ModificationType::default(),
                ModificationType::default(),
                ModificationType::default(),
            );
            let mod_fitting = FittingModifications::new(
                ModificationType::FittingCost(100.0),
                ModificationType::default(),
                ModificationType::default(),
                ModificationType::default(),
            );
            let expected = Fitting::new(200.0, 250.0, 400, 375.0);
            let actual = FITTING_STATS.apply(vec![&mod_fitting, &mod_multi]);
            println!(
                "cpu {}, pg: {}, calibration: {}, cargo: {}",
                actual.cpu, actual.pg, actual.calibration, actual.cargo
            );
            assert_partial_eq!(expected, actual);
        }
    }
}
