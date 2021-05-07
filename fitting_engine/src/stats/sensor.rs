use crate::stats::Stat;
use assertable::Assertable;
use fitting_engine_macros::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ts")]
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Stat, Deserialize, Assertable)]
#[cfg_attr(feature = "ts", derive(TS))]
pub struct Sensor {
    pub targeting_range: f64,
    pub scan_res: f64,
    pub sensor_strength: f64,
    pub max_locked_targets: u8,
}

impl Sensor {
    pub fn new(
        targeting_range: f64,
        scan_res: f64,
        sensor_strength: f64,
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
