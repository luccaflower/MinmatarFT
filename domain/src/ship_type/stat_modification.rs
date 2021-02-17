use num_traits::NumOps;
use std::cmp::Ordering;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct StatModification {
    pub(crate) shield_hp: ModificationType<usize>,
    pub(crate) armor_hp: ModificationType<usize>,
    pub(crate) hull_hp: ModificationType<usize>,
    pub(crate) velocity: ModificationType<usize>,
    pub(crate) agility: ModificationType<usize>,
    pub(crate) mass: ModificationType<usize>,
    pub(crate) power_grid: ModificationType<usize>,
    pub(crate) cpu: ModificationType<usize>,
}

impl StatModification {
    pub fn new(
        shield_hp: ModificationType<usize>,
        armor_hp: ModificationType<usize>,
        hull_hp: ModificationType<usize>,
        velocity: ModificationType<usize>,
        agility: ModificationType<usize>,
        mass: ModificationType<usize>,
        power_grid: ModificationType<usize>,
        cpu: ModificationType<usize>,
    ) -> Self {
        Self {
            shield_hp,
            armor_hp,
            hull_hp,
            velocity,
            agility,
            mass,
            power_grid,
            cpu,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ModificationType<T>
where
    T: NumOps + PartialEq + PartialOrd,
{
    Multiplicative(T),
    Additive(T),
}

impl<T> ModificationType<T>
where
    T: NumOps + PartialEq + PartialOrd,
{
    pub fn additive(&self) -> bool {
        match self {
            Self::Additive(_) => true,
            Self::Multiplicative(_) => false,
        }
    }

    pub fn multiplicative(&self) -> bool {
        match self {
            Self::Additive(_) => false,
            Self::Multiplicative(_) => true,
        }
    }

    pub fn apply(&self, val: T) -> T
    where
        T: Clone,
    {
        match self {
            ModificationType::Multiplicative(x) => val * x.clone(),
            ModificationType::Additive(x) => val + x.clone(),
        }
    }
}

impl<T> PartialEq for &ModificationType<T>
where
    T: NumOps + Eq + Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.additive() == other.additive() && self.deref().eq(other)
    }
}

impl<T> PartialOrd for &ModificationType<T>
where
    T: NumOps + Eq + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Eq for &ModificationType<T> where T: NumOps + Eq + Ord {}

impl<T> Ord for &ModificationType<T>
where
    T: NumOps + Eq + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        if self.additive() == other.additive() {
            self.deref().cmp(other)
        } else {
            if self.additive() {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
    }
}

impl<T> Deref for ModificationType<T>
where
    T: NumOps + Eq + Ord,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            ModificationType::Multiplicative(x) => x,
            ModificationType::Additive(x) => x,
        }
    }
}
