use crate::stats::Stat;
use fitting_engine_macros::*;
use serde::{Deserialize, Serialize};
use shoulda::Shoulda;
#[cfg(feature = "ts")]
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, Stat, Shoulda)]
#[cfg_attr(feature = "ts", derive(TS))]
pub struct Capacitor {
    pub capacitor_amount: f64,
    pub capacitor_recharge_time: f64,
    pub neut_resistance: f64,
}

impl Capacitor {
    pub fn new(
        capacitor_amount: f64,
        capacitor_recharge_time: f64,
        neut_resistance: f64,
    ) -> Self {
        Self {
            capacitor_amount,
            capacitor_recharge_time,
            neut_resistance,
        }
    }
}
