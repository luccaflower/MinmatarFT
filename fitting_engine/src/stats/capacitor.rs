use crate::stats::Stat;
use fitting_engine_macros::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ts")]
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, Stat, PartialEq)]
#[cfg_attr(feature = "ts", derive(TS))]
pub struct Capacitor {
    pub capacitor_amount: f64,
    pub capacitor_recharge_time: u64,
    pub neut_resistance: f32,
}

impl Capacitor {
    pub fn new(capacitor_amount: f64, capacitor_recharge_time: u64, neut_resistance: f32) -> Self {
        Self {
            capacitor_amount,
            capacitor_recharge_time,
            neut_resistance,
        }
    }
}
