use crate::stats::capacitor::Capacitor;
use crate::stats::defense::Defense;
use crate::stats::drone::Drone;
use crate::stats::fitting::Fitting;
use crate::stats::movement::Movement;
use crate::stats::sensor::Sensor;

#[derive(Debug, Clone)]
pub struct FitStats {
    pub fitting: Fitting,
    pub capacitor: Capacitor,
    pub defense: Defense,
    pub movement: Movement,
    pub sensor: Sensor,
    pub drone: Drone,
}

impl FitStats {
    pub fn new(
        fitting: Fitting,
        capacitor: Capacitor,
        defense: Defense,
        movement: Movement,
        sensor: Sensor,
        drone: Drone,
    ) -> Self {
        Self {
            fitting,
            capacitor,
            defense,
            movement,
            sensor,
            drone,
        }
    }
}
