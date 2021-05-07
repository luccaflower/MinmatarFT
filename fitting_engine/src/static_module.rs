use crate::stats::capacitor::CapacitorModifications;
use crate::stats::defense::DefenseModifications;
use crate::stats::drone::DroneModifications;
use crate::stats::fitting::FittingModifications;
use crate::stats::movement::MovementModifications;
use crate::stats::sensor::SensorModifications;
use assertable::Assertable;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub type FittingMod = Option<FittingModifications>;
pub type CapacitorMod = Option<CapacitorModifications>;
pub type DefenseMod = Option<DefenseModifications>;
pub type MovementMod = Option<MovementModifications>;
pub type SensorMod = Option<SensorModifications>;
pub type DroneMod = Option<DroneModifications>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Assertable)]
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
    pub fn new<T: Into<Cow<'a, str>>>(
        name: T,

        fitting: FittingMod,
        capacitor: CapacitorMod,
        passive_defense: DefenseMod,
        active_defense: DefenseMod,
        passive_movement: MovementMod,
        active_movement: MovementMod,
        passive_sensor: SensorMod,
        active_sensor: SensorMod,
        drone: DroneMod,

        module_slot: ModuleSlot,
        hard_point: Option<HardPoint>,
    ) -> Self {
        Self {
            name: name.into(),
            fitting,
            capacitor,
            passive_defense,
            active_defense,
            passive_movement,
            active_movement,
            passive_sensor,
            active_sensor,
            drone,
            module_slot,
            hard_point,
        }
    }

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Assertable)]
pub enum ModuleSlot {
    High,
    Med,
    Low,
    Rig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Assertable)]
pub enum HardPoint {
    Turret,
    Launcher,
}
