use crate::stats::Stat;
use fitting_engine_macros::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ts")]
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, Stat, PartialEq)]
#[cfg_attr(feature = "ts", derive(TS))]
pub struct Movement {
    pub max_velocity: f64,
    pub agility: f32,
    pub mass: u64,
    pub warp_speed: f32,
}

impl Movement {
    pub fn new(max_velocity: f64, agility: f32, mass: u64, warp_speed: f32) -> Self {
        Self {
            max_velocity,
            agility,
            mass,
            warp_speed,
        }
    }
}
