use crate::stats::Stat;
use fitting_engine_macros::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Stat)]
pub struct Defense {
    hull_hp: u32,
    hull_em_resists: f32,
    hull_therm_resists: f32,
    hull_kin_resists: f32,
    hull_exp_resists: f32,
    armor_hp: u32,
    armor_em_resists: f32,
    armor_therm_resists: f32,
    armor_kin_resists: f32,
    armor_exp_resists: f32,
    shield_hp: u32,
    shield_em_resists: f32,
    shield_therm_resists: f32,
    shield_kin_resists: f32,
    shield_exp_resists: f32,
    sig_radius: u16,
}
