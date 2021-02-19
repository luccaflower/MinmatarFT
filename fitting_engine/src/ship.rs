use crate::faction::Faction;
use crate::ship_stats::ShipStats;
use crate::ship_type::ShipType;
use crate::stats::{capacitor::Capacitor, defense::Defense, drone::Drone, fitting::Fitting, movement::Movement, sensor::Sensor};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ship<'a> {
    pub name: Cow<'a, str>,
    pub ship_type: ShipType,
    pub faction: Faction,

    pub high_slots: u8,
    pub med_slots: u8,
    pub low_slots: u8,
    pub turret_hard_points: u8,
    pub launcher_hard_points: u8,
    pub rig_slots: u8,
    pub rig_size: RigSize,

    pub fitting_stats: Fitting,
    pub defensive_stats: Defense,
    pub movement_stats: Movement,
    pub sensor_stats: Sensor,
    pub drone_stats: Drone,
    pub capacitor_stats: Capacitor,
}

impl Ship<'_> {
    pub fn new<T: Into<Cow<'static, str>>>(
        name: T,
        ship_type: ShipType,
        faction: Faction,
        high_slots: u8,
        med_slots: u8,
        low_slots: u8,
        turret_hard_points: u8,
        launcher_hard_points: u8,
        rig_slots: u8,
        rig_size: RigSize,
        fitting_stats: Fitting,
        defensive_stats: Defensive,
        movement_stats: Movement,
        sensor_stats: Sensor,
        drone_stats: Drone,
        capacitor_stats: Capacitor,
    ) -> Self {
        Self {
            name: name.into(),
            ship_type,
            faction,
            high_slots,
            med_slots,
            low_slots,
            turret_hard_points,
            launcher_hard_points,
            rig_slots,
            rig_size,
            fitting_stats,
            defensive_stats,
            movement_stats,
            sensor_stats,
            drone_stats,
            capacitor_stats
        }
    }
}

#[derive(Debug, Clone)]
pub enum RigSize {
    Small,
    Medium,
    Large,
    Capital,
}