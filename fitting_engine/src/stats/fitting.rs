use crate::stats::Stat;
use fitting_engine_macros::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Stat, Eq, PartialEq)]
pub struct Fitting {
    pub cpu: f64,
    pub pg: f64,
    pub calibration: u8,
    pub cargo: f32,
}

impl Fitting {
    pub fn new(cpu: f64, pg: f64, calibration: u8, cargo: f32) -> Self {
        Self {
            cpu,
            pg,
            calibration,
            cargo,
        }
    }
}
