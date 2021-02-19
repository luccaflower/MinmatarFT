use crate::stats::Stat;
use fitting_engine_macros::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Stat)]
pub struct Sensor {
    targeting_range: f32,
    scan_res: u16,
    sensor_strength: SensorStrength,
    max_locked_targets: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, Stat)]
pub enum SensorStrength {
    Ladar(f32),
    Radar(f32),
    Magnetometric(f32),
    Gravimetric(f32),
}
