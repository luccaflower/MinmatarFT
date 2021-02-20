use crate::stats::Stat;
use fitting_engine_macros::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Stat, Eq, PartialEq)]
pub struct Movement {
    pub max_velocity: u32,
    pub agility: f32,
    pub mass: u64,
    pub warp_speed: f32,
}

impl Movement {
    pub fn new(max_velocity: u32, agility: f32, mass: u64, warp_speed: f32) -> Self {
        Self {
            max_velocity,
            agility,
            mass,
            warp_speed,
        }
    }
}
