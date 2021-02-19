use crate::stats::Stat;
use fitting_engine_macros::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Stat)]
pub struct Sensor {
    targeting_range: f32,
    scan_res: u16,
    sensor_strength: f32,
    max_locked_targets: u8,
}
