use crate::stats::Stat;
use assertable::Assertable;
use fitting_engine_macros::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ts")]
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, Stat, PartialEq, Assertable)]
#[cfg_attr(feature = "ts", derive(TS))]
pub struct Drone {
    pub control_range: u32,
    pub capacity: u16,
    pub bandwidth: u16,
    pub max_drones: u8,
}

impl Drone {
    pub fn new(
        control_range: u32,
        capacity: u16,
        bandwidth: u16,
        max_drones: u8,
    ) -> Self {
        Self {
            control_range,
            capacity,
            bandwidth,
            max_drones,
        }
    }
}
