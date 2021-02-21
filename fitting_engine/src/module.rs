use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use crate::stats::fitting::FittingModifications;
use crate::stats::capacitor::CapacitorModifications;
use crate::stats::defense::DefenseModifications;
use crate::stats::movement::MovementModifications;
use crate::stats::sensor::SensorModifications;
use crate::stats::drone::DroneModifications;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module<'a, CA, HP, MV, M, WS> {
    pub name: Cow<'a, str>,

    pub passive_fitting: Option<FittingModifications<f64,f64,u16,f32>>,
    pub active_fitting: Option<FittingModifications<f64,f64,u16,f32>>,
    pub passive_capacitor: Option<CapacitorModifications<CA,f64,f64>>,
    pub active_capacitor: Option<CapacitorModifications<CA,f64,f64>>,
    pub passive_defense: Option<DefenseModifications<HP,f64,f64,f64,f64,HP,f64,f64,f64,f64,HP,f64,f64,f64,f64,f64>>,
    pub active_defense: Option<DefenseModifications<HP,f64,f64,f64,f64,HP,f64,f64,f64,f64,HP,f64,f64,f64,f64,f64>>,
    pub passive_movement: Option<MovementModifications<MV,f64,M,WS>>,
    pub active_movement: Option<MovementModifications<MV,f64,M,WS>>,
    pub passive_sensor: Option<SensorModifications<f64,f64,f64,u8>>,
    pub active_sensor: Option<SensorModifications<f64,f64,f64,u8>>,
    pub passive_drone: Option<DroneModifications<u32,u16,u16,u8>>,
    pub active_drone: Option<DroneModifications<u32,u16,u16,u8>>,

    pub module_slot: ModuleSlot,
    pub hard_point: Option<HardPoint>,
}

impl<'a, CA, HP, MV, M, WS> Module<'a, CA, HP, MV, M, WS> {
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