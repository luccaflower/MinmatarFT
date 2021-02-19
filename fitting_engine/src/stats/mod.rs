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
