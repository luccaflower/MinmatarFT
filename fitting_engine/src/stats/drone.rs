use crate::stats::Stat;
use fitting_engine_macros::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Stat)]
pub struct Drone {
    control_range: f32,
    capacity: u16,
    bandwidth: u16,
    max_drones: u8,
}
