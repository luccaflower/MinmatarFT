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
pub struct StaticModule<'a> {
    pub name: Cow<'a, str>,

    pub fitting: FittingMod,
    pub capacitor: CapacitorMod,
    pub passive_defense: DefenseMod,
    pub active_defense: DefenseMod,
    pub passive_movement: MovementMod,
    pub active_movement: MovementMod,
    pub passive_sensor: SensorMod,
    pub active_sensor: SensorMod,
    pub drone: DroneMod,

    pub module_slot: ModuleSlot,
    pub hard_point: Option<HardPoint>,
}

impl<'a> StaticModule<'a> {
    pub fn active(&self) -> bool {
        self.active_defense.is_some()
            || self.active_movement.is_some()
            || self.active_sensor.is_some()
    }

    pub fn active_mods(
        &self,
    ) -> (
        &FittingMod,
        &CapacitorMod,
        &DefenseMod,
        &MovementMod,
        &SensorMod,
        &DroneMod,
    ) {
        (
            &self.fitting,
            &self.capacitor,
            &self.active_defense,
            &self.active_movement,
            &self.active_sensor,
            &self.drone,
        )
    }

    pub fn passive_mods(
        &self,
    ) -> (
        &FittingMod,
        &CapacitorMod,
        &DefenseMod,
        &MovementMod,
        &SensorMod,
        &DroneMod,
    ) {
        (
            &self.fitting,
            &self.capacitor,
            &self.passive_defense,
            &self.passive_movement,
            &self.passive_sensor,
            &self.drone,
        )
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
