use crate::stats::capacitor::CapacitorModifications;
use crate::stats::defense::DefenseModifications;
use crate::stats::drone::DroneModifications;
use crate::stats::fitting::FittingModifications;
use crate::stats::movement::MovementModifications;
use crate::stats::sensor::SensorModifications;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub type FittingMod = Option<FittingModifications<f64, f64, u16, f32>>;
pub type CapacitorMod = Option<CapacitorModifications<f64, f64, f64>>;
pub type DefenseMod = Option<
    DefenseModifications<
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
    >,
>;
pub type MovementMod = Option<MovementModifications<f64, f64, f64, f64>>;
pub type SensorMod = Option<SensorModifications<f64, f64, f64, u8>>;
pub type DroneMod = Option<DroneModifications<u32, u16, u16, u8>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module<'a> {
    pub name: Cow<'a, str>,

    pub passive_fitting: FittingMod,
    pub active_fitting: FittingMod,
    pub passive_capacitor: CapacitorMod,
    pub active_capacitor: CapacitorMod,
    pub passive_defense: DefenseMod,
    pub active_defense: DefenseMod,
    pub passive_movement: MovementMod,
    pub active_movement: MovementMod,
    pub passive_sensor: SensorMod,
    pub active_sensor: SensorMod,
    pub passive_drone: DroneMod,
    pub active_drone: DroneMod,

    pub module_slot: ModuleSlot,
    pub hard_point: Option<HardPoint>,
}

impl<'a> Module<'a> {
    pub fn active(&self) -> bool {
        //self.active_effect.is_some()
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModuleSlot {
    High,
    Med,
    Low,
    Rig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HardPoint {
    Turret,
    Launcher,
}
