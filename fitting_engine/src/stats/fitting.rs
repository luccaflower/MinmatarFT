use crate::stats::Stat;
use assertable::Assertable;
use fitting_engine_macros::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ts")]
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, Stat, PartialEq, Assertable)]
#[cfg_attr(feature = "ts", derive(TS))]
pub struct Fitting {
    pub cpu: f64,
    pub pg: f64,
    pub calibration: u16,
    pub cargo: f32,
}

impl Fitting {
    pub fn new(cpu: f64, pg: f64, calibration: u16, cargo: f32) -> Self {
        Self {
            cpu,
            pg,
            calibration,
            cargo,
        }
    }
}
