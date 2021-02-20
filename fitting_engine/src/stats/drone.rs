use crate::stats::Stat;
use fitting_engine_macros::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Stat, PartialEq)]
pub struct Drone {
    pub control_range: f32,
    pub capacity: u16,
    pub bandwidth: u16,
    pub max_drones: u8,
}

impl Drone {
    pub fn new(control_range: f32, capacity: u16, bandwidth: u16, max_drones: u8) -> Self {
        Self {
            control_range,
            capacity,
            bandwidth,
            max_drones,
        }
    }
}
