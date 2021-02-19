use crate::stats::Stat;
use fitting_engine_macros::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Stat)]
pub struct Movement {
    max_velocity: u32,
    agility: f32,
    mass: u64,
    warp_speed: f32,
}
