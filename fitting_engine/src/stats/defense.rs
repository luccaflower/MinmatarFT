use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Defense {
    hull_hp: u32,
    hull_resists: [f32; 4],
    armor_hp: u32,
    armor_resists: [f32; 4],
    shield_hp: u32,
    shield_resists: [f32; 4],
    sig_radius: u16,
}