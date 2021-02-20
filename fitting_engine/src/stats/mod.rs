use num_traits::{NumOps, Zero, AsPrimitive};
use std::ops::Deref;
use std::cmp::Ordering;
use serde::{Deserialize, Serialize};

pub mod capacitor;
pub mod defense;
pub mod drone;
pub mod fitting;
pub mod movement;
pub mod sensor;

pub trait Stat {
    type Input;
    fn apply(&self, stat_mods: Vec<&Self::Input>) -> Self;
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
            T: AsPrimitive<V>
    {
        match self {
            ModificationType::Multiplicative(x) => val.as_().mul(*x),
            ModificationType::Additive(x) => val.as_().add(*x),
            ModificationType::FittingCost(x) => val.as_().add(*x),
        }.as_()
    }
}
impl<T> ModificationType<T>
    where
        T: NumOps + PartialEq + PartialOrd + Zero,
{
    pub fn default() -> Self {
        Self::Additive(num_traits::identities::zero())
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
    use once_cell::sync::Lazy;
    use crate::stats::sensor::*;

    macro_rules! assert_partial_eq {
        ($expected:ident, $actual:ident) => {
            assert!($expected.eq(&$actual))
        }
    }

    pub static SENSOR_STATS: Lazy<Sensor> = Lazy::new(|| {
        Sensor::new(50.0, 200, 32.0, 5)
    });
    mod stat_modifications_are {
        use crate::stats::ModificationType;
        use crate::stats::sensor::*;
        use crate::stats::tests::SENSOR_STATS;
        use crate::stats::Stat;

        #[test]
        fn additive() {
            let modification = SensorModifications::new(ModificationType::Additive(0.0), ModificationType::Additive(50), ModificationType::Additive(0.0), ModificationType::Additive(0));
            let expected = Sensor::new(50.0, 250, 32.0, 5);
            let actual = SENSOR_STATS.apply(vec![&modification]);
            assert_partial_eq!(expected, actual);
        }

        #[test]
        fn multiplicative() {

        }

        #[test]
        fn additive_fitting_costs() {

        }
    }
}
