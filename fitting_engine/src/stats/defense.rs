use crate::stats::Stat;
use assertable::Assertable;
use fitting_engine_macros::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ts")]
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, Stat, PartialEq, Assertable)]
#[cfg_attr(feature = "ts", derive(TS))]
pub struct Defense {
    pub hull_hp: f64,
    pub hull_em_resists: f64,
    pub hull_therm_resists: f64,
    pub hull_kin_resists: f64,
    pub hull_exp_resists: f64,
    pub armor_hp: f64,
    pub armor_em_resists: f64,
    pub armor_therm_resists: f64,
    pub armor_kin_resists: f64,
    pub armor_exp_resists: f64,
    pub shield_hp: f64,
    pub shield_em_resists: f64,
    pub shield_therm_resists: f64,
    pub shield_kin_resists: f64,
    pub shield_exp_resists: f64,
    pub sig_radius: f64,
}

impl Defense {
    pub fn new(
        hull_hp: f64,
        hull_em_resists: f64,
        hull_therm_resists: f64,
        hull_kin_resists: f64,
        hull_exp_resists: f64,
        armor_hp: f64,
        armor_em_resists: f64,
        armor_therm_resists: f64,
        armor_kin_resists: f64,
        armor_exp_resists: f64,
        shield_hp: f64,
        shield_em_resists: f64,
        shield_therm_resists: f64,
        shield_kin_resists: f64,
        shield_exp_resists: f64,
        sig_radius: f64,
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
