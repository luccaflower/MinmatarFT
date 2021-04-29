use crate::stats::Stat;
use fitting_engine_macros::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Stat, PartialEq)]
pub struct Capacitor {
    pub capacitor_amount: f64,
    pub capacitor_recharge_time: u16,
    pub neut_resistance: f32,
}

impl Capacitor {
    pub fn new(
        capacitor_amount: f64,
        capacitor_recharge_time: u16,
        neut_resistance: f32,
    ) -> Self {
        Self {
            capacitor_amount,
            capacitor_recharge_time,
            neut_resistance,
        }
    }
}
