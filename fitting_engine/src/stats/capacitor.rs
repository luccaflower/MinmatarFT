use crate::stats::Stat;
use fitting_engine_macros::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Stat)]
pub struct Capacitor {
    capacitor_amount: f64,
    capacitor_recharge_time: u16,
    neut_resistance: f32,
}
