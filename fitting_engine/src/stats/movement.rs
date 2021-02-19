use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct movement {
    max_velocity: u32,
    agility: f32,
    mass: u64,
    warp_speed: u8,
}