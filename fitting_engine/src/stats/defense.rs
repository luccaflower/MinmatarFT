use crate::stats::Stat;
use fitting_engine_macros::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Stat, Eq, PartialEq)]
pub struct Defense {
    pub hull_hp: u32,
    pub hull_em_resists: f32,
    pub hull_therm_resists: f32,
    pub hull_kin_resists: f32,
    pub hull_exp_resists: f32,
    pub armor_hp: u32,
    pub armor_em_resists: f32,
    pub armor_therm_resists: f32,
    pub armor_kin_resists: f32,
    pub armor_exp_resists: f32,
    pub shield_hp: u32,
    pub shield_em_resists: f32,
    pub shield_therm_resists: f32,
    pub shield_kin_resists: f32,
    pub shield_exp_resists: f32,
    pub sig_radius: u16,
}

impl Defense {
    pub fn new(
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
    ) -> Self {
        Self {
            hull_hp,
            hull_em_resists,
            hull_therm_resists,
            hull_kin_resists,
            hull_exp_resists,
            armor_hp,
            armor_em_resists,
            armor_therm_resists,
            armor_kin_resists,
            armor_exp_resists,
            shield_hp,
            shield_em_resists,
            shield_therm_resists,
            shield_kin_resists,
            shield_exp_resists,
            sig_radius,
        }
    }
}
