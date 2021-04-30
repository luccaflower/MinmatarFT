use crate::stats::Stat;
use fitting_engine_macros::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ts")]
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, Stat, PartialEq)]
#[cfg_attr(feature = "ts", derive(TS))]
pub struct Movement {
    pub max_velocity: f64,
    pub agility: f64,
    pub mass: f64,
    pub warp_speed: f64,
}

impl Movement {
    pub fn new(
        max_velocity: f64,
        agility: f64,
        mass: f64,
        warp_speed: f64,
    ) -> Self {
        Self {
            max_velocity,
            agility,
            mass,
            warp_speed,
        }
    }
}
