use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capacitor {
    capacitor_amount: f64,
    capacitor_recharge_time: u16,
    neut_resistance: f32,
}