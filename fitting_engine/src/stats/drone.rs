use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Drone {
    control_range: f32,
    capacity: u16,
    bandwidth: u16,
    max_drones: u8,
}