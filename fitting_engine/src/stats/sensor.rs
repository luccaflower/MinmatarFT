use crate::stats::Stat;
use fitting_engine_macros::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Stat, PartialEq)]
pub struct Sensor {
    pub targeting_range: f32,
    pub scan_res: u16,
    pub sensor_strength: f32,
    pub max_locked_targets: u8,
}

impl Sensor {
    pub fn new(
        targeting_range: f32,
        scan_res: u16,
        sensor_strength: f32,
        max_locked_targets: u8,
    ) -> Self {
        Self {
            targeting_range,
            scan_res,
            sensor_strength,
            max_locked_targets,
        }
    }
}
