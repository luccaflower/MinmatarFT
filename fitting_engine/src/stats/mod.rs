use funty::IsNumber;
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
    T: IsNumber,
{
    Multiplicative(T),
    Additive(T),
    FittingCost(T),
}

impl<T> ModificationType<T>
where
    T: IsNumber,
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

    pub fn apply(&self, val: T) -> T {
        match self {
            ModificationType::Multiplicative(x) => val.mul(*x),
            ModificationType::Additive(x) => val.add(*x),
            ModificationType::FittingCost(x) => val.sub(*x),
        }
    }
}

impl<T> Default for ModificationType<T>
where
    T: IsNumber,
{
    fn default() -> Self {
        Self::Additive(T::default())
    }
}

impl<T> PartialEq for ModificationType<T>
where
    T: IsNumber,
{
    fn eq(&self, other: &Self) -> bool {
        (match (self, other) {
            (Self::Additive(_), Self::Additive(_)) => true,
            (Self::Multiplicative(_), Self::Multiplicative(_)) => true,
            (Self::FittingCost(_), Self::FittingCost(_)) => true,
            (_, _) => false,
        }) && self.deref().eq(other)
    }
}

impl<T> PartialOrd for ModificationType<T>
where
    T: IsNumber,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Additive(_), Self::Multiplicative(_)) => {
                Some(Ordering::Greater)
            }
            (Self::Multiplicative(_), Self::FittingCost(_)) => {
                Some(Ordering::Greater)
            }
            (Self::Multiplicative(_), Self::Additive(_)) => {
                Some(Ordering::Less)
            }
            (Self::FittingCost(_), Self::Multiplicative(_)) => {
                Some(Ordering::Less)
            }
            _ => {
                let a: &T = self.deref();
                a.partial_cmp(other)
            }
        }
    }
}

impl<T> Deref for ModificationType<T>
where
    T: IsNumber,
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
