use num_traits::{NumOps, Zero};
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

    pub fn apply(&self, val: T) -> T
        where
            T: Clone,
    {
        match self {
            ModificationType::Multiplicative(x) => val * x.clone(),
            ModificationType::Additive(x) => val + x.clone(),
            ModificationType::FittingCost(x) => val + x.clone(),
        }
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
