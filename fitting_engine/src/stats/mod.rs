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
mod tests;

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

impl Default for ModificationType<u64> {
    fn default() -> Self {
        Self::Additive(0u64)
    }
}

impl<T> PartialEq for ModificationType<T>
    where
        T: NumOps + PartialEq + PartialOrd,
{
    fn eq(&self, other: &Self) -> bool {
        self.additive() == other.additive() && self.deref().eq(other)
    }
}

impl<T> PartialOrd for ModificationType<T>
    where
        T: NumOps + PartialEq + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Additive(_), Self::Multiplicative(_)) => Some(Ordering::Greater),
            (Self::Multiplicative(_), Self::FittingCost(_)) => Some(Ordering::Greater),
            (Self::Multiplicative(_), Self::Additive(_)) => Some(Ordering::Less),
            (Self::FittingCost(_), Self::Multiplicative(_)) => Some(Ordering::Less),
            _ => {
                let a: &T = self.deref();
                a.partial_cmp(other)
            }
        }
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


